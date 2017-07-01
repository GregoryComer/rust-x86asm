use std::io::Write;
use ::{Instruction, Mode, Operand, OperandSize, ProcessorLevel, Reg, RegScale};
use ::instruction_buffer::{ImmediateValue, InstructionBuffer, Prefix1, Prefix2};
use ::instruction_def::{InstructionDefinition, OperandAddressingMode, OperandDescription, OpSize64Behavior };

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionEncodingError {
    InvalidMnemonic,
    NoEncoding,
    WriteFailed,
    MismatchedSize,
    AmbiguousSize,
    MismatchedEncoding,
    InvalidAddressing
}

pub fn encode_operand(buffer: &mut InstructionBuffer, def: &OperandDescription, op: &Option<Operand>, mode: Mode, proc_level: ProcessorLevel) -> Result<(), InstructionEncodingError> {
        println!("encode() op: {:?}, addr: {:?}, type: {:?}", op, def.addressing_mode, def.operand_type);
        match def.addressing_mode {
            OperandAddressingMode::A => { // Direct memory address w/ segment selector
                buffer.add_immediate(match *op.as_ref().expect("Missing operand.") {
                    Operand::MemoryAndSegment16(seg, addr) => ImmediateValue::MemoryAndSegment16(seg, addr),
                    Operand::MemoryAndSegment32(seg, addr) => ImmediateValue::MemoryAndSegment32(seg, addr),
                    _ => panic!("Invalid operand.")
                })
            },
            OperandAddressingMode::BA => {}, // DS:rAX
            OperandAddressingMode::BB => {}, // DS:rBX+AL
            OperandAddressingMode::BD => {}, // DS:rDI
            OperandAddressingMode::C => { // Reg field is control register
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::D => { // Reg field is 
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::E |     // Rm is general/mem
            OperandAddressingMode::ES => { // Rm is fpu/mem
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::EST => { // Rm is fpu
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::F => {}, // Flags
            OperandAddressingMode::G => {   // Reg is general
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::H => { // Rm is general (ignore mod)
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct_no_mod(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::I => { // Immediate
               buffer.add_immediate(match *op.as_ref().expect("Missing operand.") {
                   Operand::Literal8(n) => ImmediateValue::Literal8(n),
                   Operand::Literal16(n) => ImmediateValue::Literal16(n),
                   Operand::Literal32(n) => ImmediateValue::Literal32(n),
                   Operand::Literal64(n) => ImmediateValue::Literal64(n),
                   _ => panic!("Invalid operand.")
               });
            },
            OperandAddressingMode::J => { // Relative offset
                // TODO Maybe should factor in instruction size here? Some assemblers do. See
                // relative call instructions (offset specified from next instruction).
                buffer.add_immediate(match *op.as_ref().expect("Missing operand") {
                   Operand::Literal8(n) => ImmediateValue::Literal8(n),
                   Operand::Literal16(n) => ImmediateValue::Literal16(n),
                   Operand::Literal32(n) => ImmediateValue::Literal32(n),
                   Operand::Literal64(n) => ImmediateValue::Literal64(n),
                   _ => panic!("Invalid operand.")
                });
            },
            OperandAddressingMode::M => { // Rm is memory
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::N => { // Rm is mmx
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::O => { // Offset
                match *op.as_ref().expect("Missing operand") {
                    Operand::Memory(addr, ..) => {
                        if addr < u16::max_value() as u64 {
                            buffer.add_immediate(ImmediateValue::Literal16(addr as u16));
                            buffer.address_size_prefix = true;
                        } else {
                            buffer.add_immediate(ImmediateValue::Literal32(addr as u32));
                        }
                    },
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::P => { // Reg is mmx
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::Q => { // Rm is mmx or memory
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::R => { // Mod is general
                // Source document says that this should mean that a general purpose register
                // is encoded in the mod field. I think this is a typo, as it doesn't match
                // Intel docs. Should be the r/m field? (See mov to/from control register)
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::S => { // Reg is segment
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::SC => {}, // Stack operand
            OperandAddressingMode::T => { // Reg is test
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::U => { // Rm is XMM
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::V => { // Reg is XMM
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::W => { // Rm is XMM/mem
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::X => {}, // DS:rSI
            OperandAddressingMode::Y => {}, // ES:rDI
            OperandAddressingMode::Z => {
                if let Some(Operand::Direct(reg)) = *op {
                    buffer.opcode_add = Some(reg.get_reg_code());
                } else { panic!("Invalid operand."); }
            }, // Last 3 bits of opcode is general reg
            OperandAddressingMode::AVXVex => {
                encode_vvvv(buffer, op.as_ref().expect("Missing operand."));
            },
            OperandAddressingMode::AVXMemRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::AVXReg => {
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::AVXRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::AVXMaskedReg |
            OperandAddressingMode::MaskedMaskReg => {
                encode_reg(buffer, op.as_ref().expect("Missing operand."));
            },
            OperandAddressingMode::AVXMemBcst32Rm |
            OperandAddressingMode::AVXMemBcst64Rm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::AVXImm8 => {
                match *op.as_ref().expect("Missing operand") {
                    Operand::Direct(reg) => {
                        buffer.displacement = Some(ImmediateValue::Literal8(reg.get_reg_code() << 4))
                    },
                    _ => panic!("Invalid operand")
                }
            },
            OperandAddressingMode::AVXDestMemRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::AVXRegMaskedRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::MaskReg => {
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::MaskRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::MaskMemRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::MaskVex => {
                encode_vvvv(buffer, op.as_ref().expect("Missing operand."));
            },
            OperandAddressingMode::GeneralRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::GeneralVex => {
                encode_vvvv(buffer, op.as_ref().expect("Missing operand."));
            },
            OperandAddressingMode::BoundReg => {
                encode_reg(buffer, op.as_ref().expect("Invalid operand."));
            },
            OperandAddressingMode::BoundMemRm => {
                encode_rm(buffer, op.as_ref().expect("Missing operand"), mode);
            },
            OperandAddressingMode::Fixed => {}
        }
        Ok(())
    }

pub fn encode<W>(writer: &mut W, def: &InstructionDefinition, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<usize, InstructionEncodingError>
    where W : Write {
    let mut buffer: InstructionBuffer = Default::default(); 

    let operand_pairs = [(&def.source, &instr.source),
        (&def.source2, &instr.source2),
        (&def.source3, &instr.source3),
        (&def.destination, &instr.destination)];

    if instr.flags.lock { buffer.prefix1 = Some(Prefix1::Lock); }
    // TODO Rep prefixes

    buffer.is_two_byte_opcode = def.is_two_byte_opcode;
    buffer.primary_opcode = def.primary_opcode;
    buffer.secondary_opcode = def.secondary_opcode;
    buffer.fixed_prefix = def.fixed_prefix;
    buffer.force_vex = def.force_vex;
    buffer.force_evex = def.force_evex;

    let op_size_pref = def.force_op_size_prefix || if let Some(size_pref_acc) = operand_pairs.iter().fold(None, |acc: Option<(bool, bool)>, pair| {
       pair.0.and_then(|op_def| {
           let maybe_pair_pref = op_def.get_operand_size_prefix(pair.1, mode);
           maybe_pair_pref.and_then(|pair_pref|
               acc.map(|acc_i: (bool, bool)| (acc_i.0 & pair_pref.0, acc_i.1 & pair_pref.1) ).or(maybe_pair_pref)
           )
       }).or(acc)
    }) {
        if size_pref_acc.0 & size_pref_acc.1 { return Err(InstructionEncodingError::AmbiguousSize); }
        else if !size_pref_acc.0 & !size_pref_acc.1 { return Err(InstructionEncodingError::MismatchedEncoding); }
        else { size_pref_acc.1 }
    } else { false };

    if op_size_pref { buffer.operand_size_prefix = true; }

    let rex_w = def.op_size_64_behavior == OpSize64Behavior::Force64 || (mode == Mode::Long) && def.force_op_size_prefix || if let Some(size_pref_acc) = operand_pairs.iter().fold(None, |acc: Option<(bool, bool)>, pair| {
       pair.0.and_then(|op_def| {
           let maybe_pair_pref = pair.1.as_ref().and_then(|o| op_def.get_rex_w(o));
           maybe_pair_pref.and_then(|pair_pref|
               acc.map(|acc_i: (bool, bool)| (acc_i.0 & pair_pref.0, acc_i.1 & pair_pref.1) ).or(maybe_pair_pref)
           )
       }).or(acc)
    }) {
        if size_pref_acc.0 & size_pref_acc.1 { return Err(InstructionEncodingError::AmbiguousSize); }
        else if !size_pref_acc.0 & !size_pref_acc.1 { return Err(InstructionEncodingError::MismatchedEncoding); }
        else { size_pref_acc.1 }
    } else { false };

    buffer.op_size_64_behavior = def.op_size_64_behavior;

    if rex_w {
        buffer.operand_size_64 = true;
    }

    if def.force_addr_size_prefix {
        buffer.address_size_prefix = true;
    }
    
    // Vector size
    if def.vector_size.is_some() {
        buffer.vector_len = def.vector_size;
    } else if let Some(vector_size) = operand_pairs.iter().filter_map(|tuple| tuple.0.and_then(
                |op_def| tuple.1.as_ref().and_then(|ref op| op_def.get_vector_size(op) ) ) )
        // If there are multiple vector sizes (i.e. conversion instructions, take the largest one)
        .fold(None, |acc: Option<OperandSize>, s| match acc {
            Some(acc_size) => if s.bits() > acc_size.bits() { Some(s) } else { acc },
            None => Some(s)
        })
    {
        match vector_size {
            OperandSize::XMMword => { 
                buffer.vector_len = Some(false);
                buffer.vex_l = Some(false);
            },
            OperandSize::YMMword => {
                buffer.vector_len = Some(true);
                buffer.vex_l = Some(false);
            },
            OperandSize::ZMMword => {
                // TODO Is this correct? Rounding modes?
                buffer.vector_len = Some(false);
                buffer.vex_l = Some(true);
            },
            _ => panic!("Invalid vector size.")
        }
    } else { println!("No vector size"); }

    // Segment register
    let seg_reg_acc = operand_pairs.iter().filter(|t| t.0.map(|o| o.fixed_operand.is_none()).unwrap_or(true))
        .filter_map(|t| t.1.as_ref().and_then(|op| op.segment_reg())).fold(Ok(None), |acc, reg| 
        acc.and_then(|a| if a.map(|ac| ac == reg).unwrap_or(true) { Ok(Some(reg)) } else { Err(()) }));
    let maybe_seg_reg = seg_reg_acc.map_err(|_| InstructionEncodingError::NoEncoding)?;

    if let Some(seg_reg) = maybe_seg_reg {
        buffer.set_segment_override(seg_reg);
    }
        
    if let Some(ref op) = def.source { encode_operand(&mut buffer, op, &instr.source, mode, proc_level)?; }
    if let Some(ref op) = def.source2 { encode_operand(&mut buffer, op, &instr.source2, mode, proc_level)?; }
    if let Some(ref op) = def.source3 { encode_operand(&mut buffer, op, &instr.source3, mode, proc_level)?; }
    if let Some(ref op) = def.destination { encode_operand(&mut buffer, op, &instr.destination, mode, proc_level)?; }

    if let Some(op_ext) = def.opcode_ext {
        // Seems like the source document uses secondary opcodes as a special case of mod/rm
        // bytes in some cases? TODO Look into this more
        if def.secondary_opcode.is_none() || buffer.mod_rm_mod.is_some() || buffer.mod_rm_rm.is_some() {
            buffer.mod_rm_reg = Some(op_ext);
        }
    }

    // SAE/Rounding Mode
    if instr.flags.sae { buffer.vex_b = Some(true); }
    if let Some(ref rounding_mode) = instr.flags.rounding_mode {
        let mode = rounding_mode.get_code();
        buffer.vex_b = Some(true);
        buffer.vex_l = Some(mode & 0x2 != 0);
        buffer.vector_len = Some(mode & 0x1 != 0);
    }

    buffer.write(writer, mode)
}

fn encode_rm(buffer: &mut InstructionBuffer, op: &Operand, mode: Mode) {
    match *op {
        Operand::Direct(reg) => {
            buffer.mod_rm_mod = Some(0b11);
            buffer.mod_rm_rm = Some(reg.get_reg_code());
        },
        Operand::Indirect(base, ..) => {
            encode_indirect(buffer, Some(base), None, None, 0, mode);
        },
        Operand::AVXBroadcastIndirect(_, base, ..) => {
            encode_indirect(buffer, Some(base), None, None, 0, mode);
            buffer.vex_b = Some(true);
        },
        Operand::IndirectDisplaced(base, disp, ..) => {
            encode_indirect(buffer, Some(base), None, None, disp, mode);
        },
        Operand::AVXBroadcastIndirectDisplaced(_, base, disp, ..) => {
            encode_indirect(buffer, Some(base), None, None, disp, mode);
            buffer.vex_b = Some(true);
        },
        Operand::IndirectScaledIndexed(base, index, scale, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), 0, mode);
        },
        Operand::AVXBroadcastIndirectScaledIndexed(_, base, index, scale, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), 0, mode);
            buffer.vex_b = Some(true);
        },
        Operand::AVXBroadcastIndirectScaledIndexedDisplaced(_, base, index, scale, disp, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode);
            buffer.vex_b = Some(true);
        },
        Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode);
        },
        Operand::AVXBroadcastIndirectScaledDisplaced(_, index, scale, disp, ..) => {
            buffer.vex_b = Some(true);
            encode_indirect(buffer, None, Some(index), Some(scale), disp, mode);
        },
        Operand::IndirectScaledDisplaced(index, scale, disp, ..) => {
            encode_indirect(buffer, None, Some(index), Some(scale), disp, mode);
        },
        Operand::Memory(addr, ..) => {
            encode_indirect(buffer, None, None, None, addr, mode);
        },
        Operand::AVXDestination(reg, maybe_mask, maybe_merge_mode) => {
            buffer.mod_rm_mod = Some(0b11);
            buffer.mod_rm_rm = Some(reg.get_reg_code());
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirect(base, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, Some(base), None, None, 0, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirectScaledIndexed(base, index, scale, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), 0, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirectScaledIndexedDisplaced(base, index, scale, disp, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirectScaledIndexedDisplaced(base, index, scale, disp, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirectScaledDisplaced(index, scale, disp, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, None, Some(index), Some(scale), disp, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::AVXDestinationIndirectDisplaced(index, disp, _, _, maybe_mask, maybe_merge_mode) => {
            encode_indirect(buffer, None, Some(index), None, disp, mode);
            buffer.mask_reg = maybe_mask.map(|m| m.get_reg_code());
            buffer.merge_mode = maybe_merge_mode;
        },
        Operand::Literal8(..) |
        Operand::Literal16(..) |
        Operand::Literal32(..) |
        Operand::Literal64(..) |
        Operand::MemoryAndSegment16(..) |
        Operand::MemoryAndSegment32(..) => panic!("Invalid addressing.")
    }
}

fn encode_indirect(buffer: &mut InstructionBuffer, base: Option<Reg>, index: Option<Reg>, scale: Option<RegScale>, displacement: u64, mode: Mode) -> Result<(), InstructionEncodingError> {
    match mode {
        Mode::Real => {
            encode_indirect_16(buffer, base, index, displacement, mode).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_32(buffer, base, index, scale, displacement, mode)
            })
        },
        Mode::Protected => {
            encode_indirect_32(buffer, base, index, scale, displacement, mode).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_16(buffer, base, index, displacement, mode)
            })
        },
        Mode::Long => {
            encode_indirect_64(buffer, base, index, scale, displacement, mode).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_32(buffer, base, index, scale, displacement, mode)
            })
        }
    }
}

fn encode_indirect_16(buffer: &mut InstructionBuffer, reg1: Option<Reg>, reg2: Option<Reg>, displacement: u64, mode: Mode) -> Result<(), InstructionEncodingError> {
    println!("enc_ind_16. reg1: {:?}, reg2: {:?}, disp: {:?}", reg1, reg2, displacement);
    let rm = match (reg1, reg2) {
        (Some(Reg::BX), Some(Reg::SI))  => 0,
        (Some(Reg::BX), Some(Reg::DI))  => 1,
        (Some(Reg::BP), Some(Reg::SI))  => 2,
        (Some(Reg::BP), Some(Reg::DI))  => 3,
        (Some(Reg::SI), None)           => 4,
        (Some(Reg::DI), None)           => 5,
        (Some(Reg::BP), None) |
        (None, None)                    => 6,
        (Some(Reg::BX), None)           => 7,
        _ => return Err(InstructionEncodingError::InvalidAddressing)
    };

    buffer.mod_rm_rm = Some(rm);

    if (displacement == 0) && !(reg1 == Some(Reg::BP) && reg2.is_none()) {
        buffer.mod_rm_mod = Some(0);
    } else if (rm == 6) && reg1.is_none() && reg2.is_none() {
        buffer.mod_rm_mod = Some(0);
        buffer.displacement = Some(ImmediateValue::Literal16(displacement as u16));
    } else if displacement < u8::max_value() as u64 {
        buffer.mod_rm_mod = Some(1);
        buffer.displacement = Some(ImmediateValue::Literal8(displacement as u8));
    } else {
        buffer.mod_rm_mod = Some(2);
        buffer.displacement = Some(ImmediateValue::Literal16(displacement as u16));
    }
    
    Ok(())
}

fn encode_indirect_32(buffer: &mut InstructionBuffer, base: Option<Reg>, index: Option<Reg>, scale: Option<RegScale>, displacement: u64, mode: Mode) -> Result<(), InstructionEncodingError> {
    fn disp_helper(buffer: &mut InstructionBuffer, disp: u64) {
        if disp == 0 { buffer.mod_rm_mod = Some(0); }
        else if disp <= u8::max_value() as u64 {
            buffer.mod_rm_mod = Some(1);
            buffer.displacement = Some(ImmediateValue::Literal8(disp as u8));
        }
        else {
            buffer.mod_rm_mod = Some(2);
            buffer.displacement = Some(ImmediateValue::Literal32(disp as u32));
        }
    }

    let real_scale = scale.unwrap_or(RegScale::One); // Unwrapped scale - defaults to 1

    if real_scale != RegScale::One && index.is_none() { return Err(InstructionEncodingError::InvalidAddressing); }

    match base {
        Some(Reg::EAX) | Some(Reg::EBX) | Some(Reg::ECX) |
        Some(Reg::EDX) | Some(Reg::ESI) | Some(Reg::EDI) => {
            match index {
                Some(index_reg) if index_reg != Reg::ESP => {
                    buffer.mod_rm_rm = Some(4); // Force SIB
                    buffer.sib_base = Some(base.unwrap().get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                None => {
                    buffer.mod_rm_rm = Some(base.unwrap().get_reg_code());
                    disp_helper(buffer, displacement);
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::EBP) => {
            match index {
                Some(index_reg) if index_reg != Reg::ESP => {
                    // Mode 0 for EBP is reserved for index*scale + disp32, so we'll encode it with
                    // mode 1 or 2.
                    if displacement < u8::max_value() as u64 {
                        buffer.mod_rm_mod = Some(1); 
                        buffer.displacement = Some(ImmediateValue::Literal8(displacement as u8));
                    }
                    else {
                        buffer.mod_rm_mod = Some(2);
                        buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                    }
                    buffer.mod_rm_rm = Some(4); // Force SIB
                    buffer.sib_base = Some(Reg::EBP.get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                },
                None => {
                    // Mode 0 for EBP means displacement (or EIP+displacement in long mode), so use
                    // mode 1 or 2.
                    buffer.mod_rm_rm = base.map(|b| b.get_reg_code());
                    if displacement < u8::max_value() as u64 {
                        buffer.mod_rm_mod = Some(1);
                        buffer.displacement = Some(ImmediateValue::Literal8(displacement as u8));
                    }
                    else {
                        buffer.mod_rm_mod = Some(2);
                        buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                    }
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::ESP) => {
            buffer.mod_rm_rm = Some(4); // Force SIB
            match index {
                Some(index_reg) if index_reg != Reg::ESP => {
                    buffer.sib_base = Some(base.unwrap().get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                None if real_scale == RegScale::One => {
                    buffer.sib_base = Some(4); // Base=ESP
                    buffer.sib_index = Some(4); // Base only
                    buffer.sib_scale = Some(RegScale::One.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::EIP) if (mode == Mode::Long) && (real_scale == RegScale::One) => {
            buffer.mod_rm_mod = Some(0);
            buffer.mod_rm_rm = Some(5); // EIP
            buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
        },
        None => {
            if let Some(index_reg) = index {
                if index_reg == Reg::ESP { return Err(InstructionEncodingError::InvalidAddressing); }
                buffer.mod_rm_mod = Some(0);
                buffer.mod_rm_rm = Some(4);
                buffer.sib_base = Some(5);
                buffer.sib_index = Some(index_reg.get_reg_code());
                buffer.sib_scale = Some(real_scale.get_sib_code());
                buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
            } else {
                buffer.mod_rm_mod = Some(0);
                buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                if mode == Mode::Protected {
                    buffer.mod_rm_rm = Some(5);
                } else { // Long mode
                    buffer.mod_rm_rm = Some(4); // Force SIB
                    buffer.sib_base = Some(5);
                    buffer.sib_scale = Some(4);
                }
            }
        },
        _ => return Err(InstructionEncodingError::InvalidAddressing)
    }

    Ok(())
}

fn encode_indirect_64(buffer: &mut InstructionBuffer, base: Option<Reg>, index: Option<Reg>, scale: Option<RegScale>, displacement: u64, mode: Mode) -> Result<(), InstructionEncodingError> {
    fn disp_helper(buffer: &mut InstructionBuffer, disp: u64) {
        if disp == 0 { buffer.mod_rm_mod = Some(0); }
        else if disp <= u8::max_value() as u64 {
            buffer.mod_rm_mod = Some(1);
            buffer.displacement = Some(ImmediateValue::Literal8(disp as u8));
        }
        else {
            buffer.mod_rm_mod = Some(2);
            buffer.displacement = Some(ImmediateValue::Literal32(disp as u32));
        }
    }

    let real_scale = scale.unwrap_or(RegScale::One); // Unwrapped scale - defaults to 1

    if real_scale != RegScale::One && index.is_none() { return Err(InstructionEncodingError::InvalidAddressing); }

    match base {
        Some(Reg::RAX) | Some(Reg::RBX) | Some(Reg::RCX) |
        Some(Reg::RDX) | Some(Reg::RSI) | Some(Reg::RDI) => {
            match index {
                Some(index_reg) if index_reg != Reg::RSP => {
                    buffer.mod_rm_rm = Some(4); // Force SIB
                    buffer.sib_base = Some(base.unwrap().get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                None => {
                    buffer.mod_rm_rm = Some(base.unwrap().get_reg_code());
                    disp_helper(buffer, displacement);
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::RBP) => {
            match index {
                Some(index_reg) if index_reg != Reg::RSP => {
                    // Mode 0 for EBP is reserved for index*scale + disp32, so we'll encode it with
                    // mode 1 or 2.
                    if displacement < u8::max_value() as u64 {
                        buffer.mod_rm_mod = Some(1); 
                        buffer.displacement = Some(ImmediateValue::Literal8(displacement as u8));
                    }
                    else {
                        buffer.mod_rm_mod = Some(2);
                        buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                    }
                    buffer.mod_rm_rm = Some(4); // Force SIB
                    buffer.sib_base = Some(Reg::RBP.get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                },
                None => {
                    // Mode 0 for EBP means displacement (or EIP+displacement in long mode), so use
                    // mode 1 or 2.
                    buffer.mod_rm_rm = base.map(|b| b.get_reg_code());
                    if displacement < u8::max_value() as u64 {
                        buffer.mod_rm_mod = Some(1);
                        buffer.displacement = Some(ImmediateValue::Literal8(displacement as u8));
                    }
                    else {
                        buffer.mod_rm_mod = Some(2);
                        buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                    }
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::RSP) => {
            buffer.mod_rm_rm = Some(4); // Force SIB
            match index {
                Some(index_reg) if index_reg != Reg::RSP => {
                    buffer.sib_base = Some(base.unwrap().get_reg_code());
                    buffer.sib_index = Some(index_reg.get_reg_code());
                    buffer.sib_scale = Some(real_scale.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                None if real_scale == RegScale::One => {
                    buffer.sib_base = Some(4); // Base=ESP
                    buffer.sib_index = Some(4); // Base only
                    buffer.sib_scale = Some(RegScale::One.get_sib_code());
                    disp_helper(buffer, displacement);
                },
                _ => return Err(InstructionEncodingError::InvalidAddressing)
            }
        },
        Some(Reg::RIP) if (real_scale == RegScale::One) => {
            buffer.mod_rm_mod = Some(0);
            buffer.mod_rm_rm = Some(5); // RIP
            buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
        },
        None => {
            if let Some(index_reg) = index {
                if index_reg == Reg::RSP { return Err(InstructionEncodingError::InvalidAddressing); }
                buffer.mod_rm_mod = Some(0);
                buffer.mod_rm_rm = Some(4);
                buffer.sib_base = Some(5);
                buffer.sib_index = Some(index_reg.get_reg_code());
                buffer.sib_scale = Some(real_scale.get_sib_code());
                buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
            } else {
                buffer.mod_rm_mod = Some(0);
                buffer.displacement = Some(ImmediateValue::Literal32(displacement as u32));
                buffer.mod_rm_rm = Some(4); // Force SIB
                buffer.sib_base = Some(5);
                buffer.sib_scale = Some(4);
            }
        },
        _ => return Err(InstructionEncodingError::InvalidAddressing)
    }

    Ok(())
}

fn encode_rm_direct_no_mod(buffer: &mut InstructionBuffer, reg: Reg) {
    buffer.mod_rm_rm = Some(reg.get_reg_code());
}

fn encode_rm_indirect(buffer: &mut InstructionBuffer, mode: Mode, reg: Reg) {
    match mode {
        Mode::Protected => match reg {
            Reg::EAX |
            Reg::EBX |
            Reg::ECX |
            Reg::EDX |
            Reg::ESI |
            Reg::EDI => {
                buffer.mod_rm_mod = Some(0b00); // Indirect operand
                buffer.mod_rm_rm = Some(reg.get_reg_code());
            },
            Reg::ESP => {
                buffer.mod_rm_mod = Some(0b00);
                buffer.mod_rm_rm = Some(0b100);
                buffer.sib_scale = Some(0b00);
                buffer.sib_base = Some(0b100);
                buffer.sib_index = Some(0b100);
            },
            Reg::EBP => {
                buffer.mod_rm_mod = Some(0b01);
                buffer.mod_rm_rm = Some(0b101);
                buffer.displacement = Some(ImmediateValue::Literal8(0));
            },
            _ => panic!("Invalid operand.")
        },
        _ => panic!("TODO")
    }
}

fn encode_rm_indirect_displaced(buffer: &mut InstructionBuffer, reg: Reg, disp: u64) {
    if disp <= <u8>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b01); // Indirect operand (8-bit displacement)
        buffer.displacement = Some(ImmediateValue::Displacement8(disp as u8));
    } else if disp <= <u32>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b10); // Indirect operand (32-bit displacement)
        buffer.displacement = Some(ImmediateValue::Displacement32(disp as u32));
    } else { panic!("Displacement too large."); }
    buffer.mod_rm_rm = Some(reg.get_reg_code());

    match reg {
        Reg::ESP => {
            buffer.sib_scale = Some(0b00);
            buffer.sib_base = Some(0b100);
            buffer.sib_index = Some(0b100);
        },
        _ => {}
    }
}

fn encode_rm_indirect_scaled_indexed(buffer: &mut InstructionBuffer, base: Reg, index: Reg, scale: RegScale) {
    if base != Reg::EBP {
    buffer.mod_rm_mod = Some(0b00); // Indirect operand
    buffer.mod_rm_rm = Some(0b100); // SIB
    buffer.sib_base = Some(base.get_reg_code());
    buffer.sib_index = Some(index.get_reg_code());
    buffer.sib_scale = Some(scale.get_sib_code());
    } else { // EBP
        buffer.mod_rm_mod = Some(0b01);
        buffer.mod_rm_rm = Some(0b100);
        buffer.sib_base = Some(base.get_reg_code());
        buffer.sib_index = Some(index.get_reg_code());
        buffer.sib_scale = Some(scale.get_sib_code());
        buffer.displacement = Some(ImmediateValue::Literal8(0));
    }
}

fn encode_rm_indirect_scaled_indexed_displaced(buffer: &mut InstructionBuffer, base: Reg, index: Reg, scale: RegScale, disp: u64) {
    if disp <= <u8>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b01); // Indirect operand (8-bit displacement)
        buffer.displacement = Some(ImmediateValue::Displacement8(disp as u8));
    } else if disp <= <u32>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b10); // Indirect operand (32-bit displacement)
        buffer.displacement = Some(ImmediateValue::Displacement32(disp as u32));
    } else { panic!("Displacement too large for 32-bit mode."); }
    buffer.mod_rm_rm = Some(0b100); // SIB
    buffer.sib_base = Some(base.get_reg_code());
    buffer.sib_index = Some(index.get_reg_code());
    buffer.sib_scale = Some(scale.get_sib_code());
}

fn encode_rm_indirect_scaled_displaced(buffer: &mut InstructionBuffer, reg: Reg, scale: RegScale, disp: u64) {
    if reg == Reg::ESP { panic!("Invalid operand"); }
    buffer.mod_rm_mod = Some(0b00);
    buffer.mod_rm_rm = Some(0b100);
    buffer.sib_base = Some(0b101);
    buffer.sib_index = Some(reg.get_reg_code());
    buffer.sib_scale = Some(scale.get_sib_code());
    buffer.displacement = Some(ImmediateValue::Displacement32(disp as u32));
}

fn encode_rm_memory(buffer: &mut InstructionBuffer, addr: u64) {
    buffer.mod_rm_mod = Some(0b00); // Indirect operand
    buffer.mod_rm_rm = Some(0b101); // Fixed address
    buffer.displacement = Some(ImmediateValue::Displacement32(addr as u32));
}

fn encode_reg(buffer: &mut InstructionBuffer, op: &Operand) {
    match *op {
        Operand::Direct(reg) => { buffer.mod_rm_reg = Some(reg.get_reg_code()); }
        Operand::AVXDestination(reg, maybe_mask, maybe_merge_mode) => {
            buffer.mod_rm_reg = Some(reg.get_reg_code());
            if let Some(mask) = maybe_mask {
                buffer.mask_reg = Some(mask.get_reg_code());
            }
            if let Some(merge_mode) = maybe_merge_mode {
                buffer.merge_mode = Some(merge_mode);
            }
        },
        _ => panic!("Invalid operand.")
    }
}

fn encode_vvvv(buffer: &mut InstructionBuffer, op: &Operand) {
    match *op {
        Operand::Direct(reg) => { buffer.vex_operand = Some(reg.get_reg_code()); }
        _ => panic!("Invalid operand.")
    }
}
