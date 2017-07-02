/*
 *  Instruction format and definitions from ref.x86asm.net
 */

#[macro_use]
extern crate xml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate byteorder;
extern crate itertools;

mod memreader;
mod x86;

use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use std::io::Read;
use std::sync::Mutex;
use xml::common::Position;
use x86::{Mnemonic, Mode, ProcessorLevel };
use x86::instruction_def::{ OperandType, OperandAddressingMode, RingLevel, InstructionDefinitionFlags, InstructionDefinition, OperandDescription, FixedOperand, OpSize64Behavior };

lazy_static! {
    static ref X86_INSTR_DEFS : Mutex<HashMap<Mnemonic, Vec<InstructionDefinition>>> = { Mutex::new(HashMap::new()) };
}

pub fn main() {
    let instr_data = include_str!("x86reference.xml");
    let mut xml_reader = EventReader::from_str(instr_data);
    if let Err(val) = parse_instructions(&mut xml_reader) {
        println!("Parse error ({:?}): {}", xml_reader.position(), val);
    } else {
        //println!("Parse successful.");
    }
}

fn parse_instructions<R: Read>(reader: &mut EventReader<R>) -> Result<(), &'static str>{
    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "one-byte" => { parse_instruction_container(reader, false)? },
                    "two-byte" => { parse_instruction_container(reader, true)? },
                    _ => {}
                }
            },
            XmlEvent::EndElement { name, .. } => {
                match name.local_name.as_ref() {
                   "x86reference" => { break },
                   _ => {}
                }
            },
            _ => {}
        }
    }
    Ok(())
}

fn parse_instruction_container<R: Read>(reader: &mut EventReader<R>, two_byte_opcodes: bool) -> Result<(), &'static str> {
    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "pri_opcd" => {
                        let opcode = get_opcode_attr(&attributes)?;
                        parse_opcode(reader, opcode, two_byte_opcodes)?;
                    },
                    _ => {}
                }
            },
            XmlEvent::EndElement { name, .. } => {
                match name.local_name.as_ref() {
                    "one-byte" => { break; },
                    "two-byte" => { break; },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    Ok(())
}



fn parse_opcode<R: Read>(reader: &mut EventReader<R>, opcode: u8, is_two_byte_opcode: bool) -> Result<(), &'static str> {
    let mut entries: Vec<InstructionDefinition> = Vec::new();
    let mut valid_in_long_mode = true;

    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "entry" => {
                        // In the operand2 document, there is a entry indicating that 0F functions as
                        // the two-byte opcode prefix. We don't want to parse this.
                        if !attr_has_value(&attributes, "ref", "two-byte") {
                            // Source document indicates that opcodes are invalid in long mode with
                            // special entries.
                            let is_invalid = attr_has_value(&attributes, "attr", "invd");
                            if is_invalid && attr_has_value(&attributes, "mode", "e") {
                                valid_in_long_mode = false;
                            } else if !is_invalid { // TODO Could handle other invalid blocks. May not need to, though
                                let mut new_entries = parse_opcode_entry(reader, &attributes, opcode, is_two_byte_opcode)?;
                                while let Some(e) = new_entries.pop() {
                                    entries.push(e);
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },
            XmlEvent::EndElement { name, .. } => {
                match name.local_name.as_ref() {
                    "pri_opcd" => { break; },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    let mut defs = X86_INSTR_DEFS.lock().unwrap();
    for mut entry in entries {
        entry.valid_in_long_mode = valid_in_long_mode;
        let mut list = if defs.contains_key(&entry.mnemonic) {
            defs.get_mut(&entry.mnemonic).unwrap()
        } else {
            let new_list = Vec::new();
            defs.insert(entry.mnemonic, new_list);
            defs.get_mut(&entry.mnemonic).unwrap() // Not sure if there's a better way to do this. Can't return &new_list, since it's moved into the hashmap.
        };
        println!("{:?},", entry); 
        list.push(entry);
    }

    Ok(())
}

fn parse_opcode_entry<R: Read>(reader: &mut EventReader<R>, attributes: &Vec<OwnedAttribute>, opcode: u8, is_two_byte_opcode: bool) -> Result<Vec<InstructionDefinition>, &'static str> {
    // Don't parse undocumented instructions
    if get_attr_as_bool(&attributes, "is_undoc")?.unwrap_or(false) { return Ok(Vec::new()); }
    if attr_has_value(&attributes, "doc", "m") { return Ok(Vec::new()); }

    let mut syntax_blocks = Vec::new();
    let mut secondary_opcode: Option<u8> = None;
    let mut prefix: Option<u8> = None;
    let mut opcode_ext: Option<u8> = None;
    let mut proc_start: Option<ProcessorLevel> = None;
    let mut proc_end: Option<ProcessorLevel> = None;

    let ring_level: Option<RingLevel> = get_attr_val(&attributes, "ring").map_or(Ok(None), |val| parse_ring_level(val).map(Some) )?;
    let mode: Option<Mode> = get_attr_val(&attributes, "mode").map_or(Ok(None), |val| parse_mode(val).map(Some) )?;
    let can_lock = get_attr_as_bool(&attributes, "lock")?;
    let mem_format = get_attr_as_bin(&attributes, "mem_format")?;
    let tttn = get_attr_as_bin(&attributes, "tttn")?;

    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                   "syntax" => {
                        syntax_blocks.push(parse_syntax_block(reader)?);
                   }
                   "opcd_ext" => {
                       if let Ok(XmlEvent::Characters(str)) = reader.next() {
                           opcode_ext = u8::from_str_radix(str.as_ref(), 16)
                               .map(Some).map_err(|_| "Invalid opcode extention.")?;
                       }
                    },
                    "sec_opcd" => {
                       if let Ok(XmlEvent::Characters(str)) = reader.next() {
                           secondary_opcode = u8::from_str_radix(str.as_ref(), 16)
                               .map(Some).map_err(|_| "Invalid secondary opcode.")?;
                       }
                    },
                    "pref" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                           prefix = u8::from_str_radix(str.as_ref(), 16)
                               .map(Some).map_err(|_| "Invalid prefix tag")?;
                        }
                    }
                   "proc_start" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                            proc_start = parse_proc_level(str.as_ref()).map(Some)?;
                        }
                    },
                    "proc_end" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                            proc_end = parse_proc_level(str.as_ref()).map(Some)?;
                        }
                    },
                    _ => {}
                }
            },
            XmlEvent::EndElement { name, .. } => {
                match name.local_name.as_ref() {
                    "entry" => { break; },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    Ok(syntax_blocks.iter().map(|block| {
        InstructionDefinition {
            fixed_prefix: prefix,
            is_two_byte_opcode: is_two_byte_opcode,
            primary_opcode: opcode,
            secondary_opcode: secondary_opcode,
            flags: InstructionDefinitionFlags {
                tttn: tttn,
                mem_format: mem_format
            },
            opcode_ext: opcode_ext,
            has_destination: block.has_destination,
            proc_start: proc_start,
            proc_end: proc_end,
            mode: mode.unwrap_or(Mode::Real),
            ring_level: ring_level.unwrap_or(RingLevel::Ring3),
            can_lock: can_lock.unwrap_or(false),
            valid_in_long_mode: true,
            force_op_size_prefix: false,
            force_addr_size_prefix: false,
            op_size_64_behavior: OpSize64Behavior::Normal,
            force_vex: false,
            force_evex: false,
            vector_size: None,
            allow_rounding_mode: false,
            allow_sae: false,
            allowed_broadcast: None,
            mnemonic: block.mnemonic,
            operand1: block.operand1,
            operand2: block.operand2,
            operand3: block.operand3,
            operand4: block.operand4,
        }
    }).collect())
}

struct SyntaxBlock {
    mnemonic: Mnemonic,
    operand1: Option<OperandDescription>,
    operand2: Option<OperandDescription>,
    operand3: Option<OperandDescription>,
    operand4: Option<OperandDescription>,
    has_destination: bool
}

fn parse_syntax_block<R: Read>(reader: &mut EventReader<R>) -> Result<SyntaxBlock, &'static str> {
    let mut mnemonic: Option<Mnemonic> = None;
    let mut operand1: Option<OperandDescription> = None;
    let mut operand2: Option<OperandDescription> = None;
    let mut operand3: Option<OperandDescription> = None;
    let mut operand4: Option<OperandDescription> = None;

    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "mnem" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                            mnemonic = Some(Mnemonic::parse(str.as_ref())?);
                        } else {
                            return Err("Invalid mnemonic tag.");
                        }
                    },
                   "dst" => {
                        if get_attr_val(&attributes, "displayed").is_none() {
                            operand1 = parse_operand_description(reader, &attributes).map(Some)?;
                        }
                    },
                    "src" => {
                        if get_attr_val(&attributes, "displayed").is_none() {
                            let operand = parse_operand_description(reader, &attributes).map(Some)?;
                            if operand2.is_none() { operand2 = operand; }
                            else if operand3.is_none() { operand3 = operand; }
                            else if operand4.is_none() { operand4 = operand; }
                            else { return Err("Can't process more than three operand2 operands."); }
                        }
                    },
                    "syntax" => { break; },
                   _ => {}
                }
            },
            XmlEvent::EndElement { name, .. } => {
                match name.local_name.as_ref() {
                    "syntax" => { break; },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    // Parsing logic treats operand1 as operand1. If no operand1 operand
    // exists, but there are other operands, shift the operands down so that
    // the first operand is operand1.
    let has_destination = if operand1.is_none() && operand2.is_some() {
        operand1 = operand2;
        operand2 = operand3;
        operand3 = operand4;
        operand4 = None;
        false
    } else { true };

    Ok(SyntaxBlock {
        mnemonic: mnemonic.ok_or("Syntax block is missing mnemonic.")?,
        operand1: operand1,
        operand2: operand2,
        operand3: operand3,
        operand4: operand4,
        has_destination: has_destination
    })
}

fn parse_mode(val: &str) -> Result<Mode, &'static str> {
    Ok(match val {
        "r" => Mode::Real,
        "p" => Mode::Protected,
        "e" => Mode::Long,
        "s" => Mode::SystemManagement,
        _ => return Err("Unknown mode.")
    })
}

fn parse_ring_level(val: &str) -> Result<RingLevel, &'static str> {
   Ok(match val {
       "0" => RingLevel::Ring0,
       "1" => RingLevel::Ring1,
       "2" => RingLevel::Ring2,
       "3" => RingLevel::Ring3,
       "f" => RingLevel::Ring0, // TODO We could handle conditional ring level instructions better
       _ => return Err("Invalid ring level.")
   })
}

fn parse_proc_level(val: &str) -> Result<ProcessorLevel, &'static str> {
    val.parse::<u8>().map(|n| Ok(match n {
        0 => ProcessorLevel::i8086,
        1 => ProcessorLevel::i80186,
        2 => ProcessorLevel::i80286,
        3 => ProcessorLevel::i80386,
        4 => ProcessorLevel::i80486,
        5 => ProcessorLevel::Pentium1,
        6 => ProcessorLevel::Pentium1Mmx,
        7 => ProcessorLevel::PentiumPro,
        8 => ProcessorLevel::Pentium2,
        9 => ProcessorLevel::Pentium3,
        10 => ProcessorLevel::Pentium4,
        11 => ProcessorLevel::Core1,
        12 => ProcessorLevel::Core2,
        13 => ProcessorLevel::Corei7,
        99 => return Err("Itanium instructions are not supported."),
        _ => return Err("Unknown processor level.")
    })).map_err(|_| "Invalid processor level value.")?
}

fn parse_operand_description<R: Read>(reader: &mut EventReader<R>, attributes: &Vec<OwnedAttribute>) -> Result<OperandDescription, &'static str> { 
    let mut addressing_mode: Option<OperandAddressingMode> = None;
    let mut operand_type: Option<OperandType> = None;
    let mut fixed_operand: Option<FixedOperand> = None;

    if let Some(op_type) = get_attr_val(&attributes, "type") {
        operand_type = Some(parse_operand_type(op_type)?);
    }

    while let Ok(e) = reader.next() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "a" | "address" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                            addressing_mode = Some(parse_addressing_mode(str.as_ref())?);
                        } else {
                            return Err("Invalid operand addressing mode.");
                        }
                    },
                    "t" | "type" => {
                        if let Ok(XmlEvent::Characters(str)) = reader.next() {
                            operand_type = Some(parse_operand_type(str.as_ref())?);
                        } else {
                            return Err("Invalid operand type.");
                        }
                    },
                    "src" | "dst" => {
                        
                    }
                    _ => {}
                }
            },
            XmlEvent::Characters(str) => {
                fixed_operand = Some(match str.as_ref() {
                    "AL" => FixedOperand::AL,
                    "AX" => FixedOperand::AX,
                    "eAX" => FixedOperand::ASized32,
                    "rAX" => FixedOperand::ASized,
                    "EAX" => FixedOperand::EAX,
                    "CL" => FixedOperand::CL,
                    "DX" => FixedOperand::DX,
                    "CS" => FixedOperand::CS,
                    "DS" => FixedOperand::DS,
                    "ES" => FixedOperand::ES,
                    "FS" => FixedOperand::FS,
                    "GS" => FixedOperand::GS,
                    "SS" => FixedOperand::SS,
                    "ST" => FixedOperand::ST,
                    "ST1" => FixedOperand::ST1,
                    "[DI]" => FixedOperand::DIIndirect,
                    "[SI]" => FixedOperand::SIIndirect,
                    "[rDI]" => FixedOperand::DIIndirectSized,
                    "[rSI]" => FixedOperand::SIIndirectSized,
                    // TODO There's a probably a difference in the two forms below, need to figure
                    // out what it is
                    "(DS:)[rBX+AL]" => FixedOperand::BSizedALIndirectDS,
                    "(DS:)[SI]" | "DS:[SI]" => FixedOperand::SIIndirectDS,
                    "(DS:)[rSI]" | "DS:[rSI]" | "(DS):[rSI]" => FixedOperand::SIIndirectSizedDS,
                    "(ES:)[DI]" | "ES:[DI]" => FixedOperand::DIIndirectES,
                    "(ES:)[rDI]" | "ES:[rDI]" => FixedOperand::DIIndirectSizedES,
                    "(DS:)[rDI]" => FixedOperand::DIIndirectSizedDS,
                    "SS:[rSP]" => FixedOperand::SPIndirectSizedSS,
                    "(DS:)[rAX]" => FixedOperand::AIndirectSizedDS,
                    // TODO Maybe could look into use cases for flags as operand (I think most are
                    // implict. Maybe this needs to discriminate between the forms?
                    "Flags" | "eFlags" | "EFlags" | "RFlags" => FixedOperand::Flags,
                    "GDTR" => FixedOperand::GDTR,
                    "LDTR" => FixedOperand::LDTR,
                    "IDTR" => FixedOperand::IDTR,
                    "TR" => FixedOperand::TR,
                    "XCR" => FixedOperand::XCR,
                    "MSR" => FixedOperand::MSR,
                    "PMC" => FixedOperand::PMC,
                    _ => {
                        // Look for immediate operands
                        if let Ok(val) = str.parse::<u8>() {
                            FixedOperand::Literal8(val)
                        } else {
                            println!("{}", str);
                            return Err("Invalid fixed operand")
                        }
                    }
                });
            },
            XmlEvent::EndElement { name, .. } => {
                if name.local_name == "src" || name.local_name == "dst" {
                    break;
                }
            },
            _ => {}
        }
    }

    if operand_type.is_none() {
        // This is a special case to handle LEA, since the operand type doesn't actually matter, only
        // the address.
        if let Some(OperandAddressingMode::M) = addressing_mode {
            operand_type = Some(OperandType::UnsizedMemory);
        }
        
        // This is a special case to handle INT 3. May need to be made more flexible in the
        // future.
        if let Some(FixedOperand::Literal8(..)) = fixed_operand {
            operand_type = Some(OperandType::B);
        }

        // Set operand type for fixed fpu operandss.
        match fixed_operand {
            Some(FixedOperand::ST) => {
                operand_type = Some(OperandType::FpuRegister);
            }
            _ => {}
        }
    }

    // Source document doesn't specify an operand type for fpu registers.
    if let Some(OperandAddressingMode::EST) = addressing_mode {
        operand_type = Some(OperandType::FpuRegister); 
    }
    
    // Addressing mode is not specified for some fixed operands in the operand2 document.
    if addressing_mode.is_none() {
        if fixed_operand.is_some() {
            addressing_mode = Some(OperandAddressingMode::Fixed);
        }
    }
    
    Ok(OperandDescription {
        addressing_mode: try!(addressing_mode.ok_or("No addressing mode specified for operand.")),
        operand_type: try!(operand_type.ok_or("No operand type specified for operand.")),
        fixed_operand: fixed_operand
    })
}

fn parse_addressing_mode(val: &str) -> Result<OperandAddressingMode, &'static str> {
    match val.trim() { 
        "A" => Ok(OperandAddressingMode::A),
        "BA" => Ok(OperandAddressingMode::BA),
        "BB" => Ok(OperandAddressingMode::BB),
        "BD" => Ok(OperandAddressingMode::BD),
        "C" => Ok(OperandAddressingMode::C),
        "D" => Ok(OperandAddressingMode::D),
        "E" => Ok(OperandAddressingMode::E),
        "ES" => Ok(OperandAddressingMode::ES),
        "EST" => Ok(OperandAddressingMode::EST),
        "F" => Ok(OperandAddressingMode::F),
        "G" => Ok(OperandAddressingMode::G),
        "H" => Ok(OperandAddressingMode::H),
        "I" => Ok(OperandAddressingMode::I),
        "J" => Ok(OperandAddressingMode::J),
        "M" => Ok(OperandAddressingMode::M),
        "N" => Ok(OperandAddressingMode::N),
        "O" => Ok(OperandAddressingMode::O),
        "P" => Ok(OperandAddressingMode::P),
        "Q" => Ok(OperandAddressingMode::Q),
        "R" => Ok(OperandAddressingMode::R),
        "S" => Ok(OperandAddressingMode::S),
        "SC" => Ok(OperandAddressingMode::SC),
        "T" => Ok(OperandAddressingMode::T),
        "U" => Ok(OperandAddressingMode::U),
        "V" => Ok(OperandAddressingMode::V),
        "W" => Ok(OperandAddressingMode::W),
        "X" => Ok(OperandAddressingMode::X),
        "Y" => Ok(OperandAddressingMode::Y),
        "Z" => Ok(OperandAddressingMode::Z),
        _ => Err("Unknown operand addressing mode.")
    }
}

fn parse_operand_type(val: &str) -> Result<OperandType, &'static str> {
    match val.trim() {
        "a" => Ok(OperandType::A),
        "b" => Ok(OperandType::B),
        "bcd" => Ok(OperandType::BCD),
        "bs" => Ok(OperandType::BS),
        "bsq" => Ok(OperandType::BSQ),
        "bss" => Ok(OperandType::BSS),
        "c" => Ok(OperandType::C),
        "d" => Ok(OperandType::D),
        "di" => Ok(OperandType::DI),
        "do" => Ok(OperandType::DO),
        "dq" => Ok(OperandType::DQ),
        "dqp" => Ok(OperandType::DQP),
        "dr" => Ok(OperandType::DR),
        "ds" => Ok(OperandType::DS),
        "e" => Ok(OperandType::E),
        "er" => Ok(OperandType::ER),
        "p" => Ok(OperandType::P),
        "pi" => Ok(OperandType::PI),
        "pd" => Ok(OperandType::PD),
        "ps" => Ok(OperandType::PS),
        "psq" => Ok(OperandType::PSQ),
        "pt" => Ok(OperandType::PT),
        "ptp" => Ok(OperandType::PTP),
        "q" => Ok(OperandType::Q),
        "qi" => Ok(OperandType::QI),
        "qp" => Ok(OperandType::QP),
        "s" => Ok(OperandType::S),
        "sd" => Ok(OperandType::SD),
        "si" => Ok(OperandType::SI),
        "sr" => Ok(OperandType::SR),
        "ss" => Ok(OperandType::SS),
        "st" => Ok(OperandType::ST),
        "stx" => Ok(OperandType::STX),
        "t" => Ok(OperandType::T),
        "v" => Ok(OperandType::V),
        "vds" => Ok(OperandType::VDS),
        "vq" => Ok(OperandType::VQ),
        "vqp" => Ok(OperandType::VQP),
        "vs" => Ok(OperandType::VS),
        "w" => Ok(OperandType::W),
        "wi" => Ok(OperandType::WI),
        "wo" => Ok(OperandType::WO),
         _ => Err("Unknown operand type.")
    }
}

fn get_opcode_attr(attrs: &Vec<OwnedAttribute>) -> Result<u8, &'static str> {
    get_attr_as_hex(attrs, "value")?.ok_or("Failed to retrieve opcode value.")
}

fn get_attr_val<'a>(attrs: &'a Vec<OwnedAttribute>, name: &str) -> Option<&'a str> {
    attrs.iter().find(|attr| attr.name.local_name == name).map(|a| a.value.as_ref())
}

fn attr_has_value(attrs: &Vec<OwnedAttribute>, name: &str, val: &str) -> bool {
    get_attr_val(attrs, name).map(|v| v == val).unwrap_or(false)
}

fn get_attr_as_hex(attrs: &Vec<OwnedAttribute>, name: &str) -> Result<Option<u8>, &'static str> {
    get_attr_val(attrs, name).map(|val| u8::from_str_radix(val, 16)
        .map_err(|_| "Attribute was not a valid u8 value"))
        .map(|r| r.map(Some) ).unwrap_or(Ok(None))
}

fn get_attr_as_bin(attrs: &Vec<OwnedAttribute>, name: &str) -> Result<Option<u8>, &'static str> {
    get_attr_val(attrs, name).map(|val| u8::from_str_radix(val, 2)
        .map_err(|_| "Attribute was not a valid u8 value"))
        .map(|r| r.map(Some) ).unwrap_or(Ok(None))
}

fn get_attr_as_bool(attrs: &Vec<OwnedAttribute>, name: &str) -> Result<Option<bool>, &'static str> {
    get_attr_val(attrs, name).map(|val| match val {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err("Invalid boolean attribute value.")
        }).map(|r| r.map(Some) ).unwrap_or(Ok(None))
}
