#[macro_use] extern crate itertools;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;

extern crate csv;
extern crate rand;
extern crate serde;

mod gen_tests;
mod instruction;
mod instruction_def;
mod operand;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::env;
use std::fmt;
use std::io;
use std::io::Write;
use std::str;
use std::vec;

use itertools::Itertools;
use nom::*;

use instruction::Reg;
use instruction_def::CompositePrefix;
use instruction_def::FixedOperand;
use instruction_def::InstructionDefinition;
use instruction_def::OperandAccess;
use instruction_def::OperandDefinition;
use instruction_def::OperandEncoding;
use instruction_def::OperandSizePrefixBehavior;
use instruction_def::OperandType;
use instruction_def::PrefixBehavior;
use instruction_def::RegType;
use instruction_def::VexOperandBehavior;
use operand::OperandSize;

const CSV_INPUT: &'static str = "/home/gregory/Downloads/x86.csv";
const TEST_OUTPUT: &'static str = "test";

#[derive(Deserialize, Debug)]
struct EncodingRecord {
    #[serde(rename="Instruction")] instruction: String,
    #[serde(rename="Opcode")] opcode: String,
    #[serde(rename="Valid 64-bit")] valid_64: String,
    #[serde(rename="Valid 32-bit")] valid_32: String,
    #[serde(rename="Valid 16-bit")] valid_16: String,
    #[serde(rename="Feature Flags")] feature_flag: String,
    #[serde(rename="Operand 1")] operand1: String,
    #[serde(rename="Operand 2")] operand2: String,
    #[serde(rename="Operand 3")] operand3: String,
    #[serde(rename="Operand 4")] operand4: String,
    #[serde(rename="Tuple Type")] tuple_type: String,
    #[serde(rename="Description")] description: String
}

fn main() {
    let args = parse_args();
    let file = File::open(CSV_INPUT).expect("Error opening input file.");
    let mut reader = csv::Reader::from_reader(file);

    // Clean & recreate output dir
    std::fs::remove_dir_all(TEST_OUTPUT);
    std::fs::create_dir(TEST_OUTPUT);

    // Track how many tests have been emitted for each mnemonic for numbering purposes
    let mut test_count = HashMap::new();

    for group in reader.deserialize().map(|r| r.expect("Error reading record."))
        .filter(|r: &EncodingRecord| args.filter.as_ref().map_or(true, 
            // |f| r.instruction.find(f.as_str()).is_some()))
            |f| r.instruction.as_str().starts_with(f)))
        .map(|ref r| parse_record(r))
        .map(|r| {
            let key = (r.mnemonic.clone(), r.primary_opcode, r.secondary_opcode);
            (r, key)
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .into_iter().group_by(|r| (r.0.mnemonic.clone(), r.0.primary_opcode, r.0.secondary_opcode))
        .into_iter().map(|(_, group)| group.map(|(g, _)| g).collect::<Vec<InstructionDefinition>>()) {

        let op_size_op = (0u8..4).filter_map(|i| {
            let (has_16, has_32) = group.iter().filter_map(|o| o.operands[i as usize].as_ref())
                .fold((false, false), |(has_16, has_32), op|
                    (has_16 || op.size == OperandSize::Word || op.size == OperandSize::Far16,
                    has_32 || op.size == OperandSize::Dword || op.size == OperandSize::Far32)
                );
            if has_16 && has_32 { Some(i) } else { None }
            // if has_16 || has_32 { Some(i) } else { None }
        }).next();
        
        for mut instr in group {
            if let Some(i) = op_size_op {
                if instr_needs_op_size_prefix(&instr, i) {
                    if instr.operands[i as usize].as_ref().map_or(false,
                        |op| op.size == OperandSize::Word || op.size == OperandSize::Far16) {
                        instr.operand_size_prefix = OperandSizePrefixBehavior::NotReal;
                    } else {
                        instr.operand_size_prefix = OperandSizePrefixBehavior::RealOnly;
                    }
                }
            }

            fixup(&mut instr);

            println!("{:?},", &instr);

            if args.emit_tests {
                gen_tests::emit_tests(&instr, TEST_OUTPUT, &mut test_count);
            }
        }
    }

    // Cleanup
    std::fs::remove_file("test.s");
    std::fs::remove_file("test.out");
}

struct ParsedArgs {
    emit_tests: bool,
    filter: Option<String>
}

fn parse_args() -> ParsedArgs {
    let mut emit_tests = false;
    let mut filter = None;

    let mut args_iter = env::args();
    args_iter.next(); // Skip path
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--tests" => { emit_tests = true; },
            "--filter" => {
                filter = Some(args_iter.next().expect("Invalid argument -- missing filter value."))
            },
            _ => panic!("Unknown argument {}.", arg)
        }
    }

    ParsedArgs { emit_tests, filter }
}

fn parse_record(record: &EncodingRecord) -> instruction_def::InstructionDefinition {
    let (instr_left, (mnemonic, instruction_parts)) = parse_instruction(record.instruction.as_bytes()).unwrap();
    let (opcode_left, opcode_parts) = parse_opcode(record.opcode.as_bytes()).unwrap();
    let operand1 = parse_operand_encoding_opt(&record.operand1);
    let operand2 = parse_operand_encoding_opt(&record.operand2);
    let operand3 = parse_operand_encoding_opt(&record.operand3);
    let operand4 = parse_operand_encoding_opt(&record.operand4);
    
    if instr_left.len() != 0 { eprintln!("record: {:?}", record); panic!("Didn't parse entire instruction: {:?}", str::from_utf8(instr_left)); }
    if opcode_left.len() != 0 { eprintln!("record: {:?}", record); panic!("Didn't parse entire opcode: {:?}", str::from_utf8(opcode_left)); } 
    let mut instr = InstructionDefinition {
        // Is there a better way to do this?
        mnemonic: String::from(str::from_utf8(mnemonic.as_bytes()).unwrap()), 

        valid_64: record.valid_64.starts_with("V"),
        valid_32: record.valid_32.starts_with("V"),
        valid_16: record.valid_16.starts_with("V"),

        ..Default::default()
    };

    // println!("rec: {:?}", record);

    let mut operands = vec![operand4, operand3, operand2, operand1];

    let mut has_embedded_reg = false;

    for op_token in opcode_parts {
        match op_token {
            OpcodeToken::NoPrefix => { instr.allow_prefix = false; }
            OpcodeToken::Vex(op_behavior, vec_size, prefix, map, we) |
            OpcodeToken::Evex(op_behavior, vec_size, prefix, map, we) => {
                match op_token {
                    OpcodeToken::Vex(..) => {
                        instr.composite_prefix = Some(CompositePrefix::Vex {
                            vector_size: vec_size,
                            operand_behavior: op_behavior,
                            we: we
                        }); }
                    OpcodeToken::Evex(..) => {
                        instr.composite_prefix = Some(CompositePrefix::Evex {
                            vector_size: vec_size,
                            operand_behavior: op_behavior,
                            we: we
                        }); }
                    _ => unreachable!()
                }
                match prefix {
                    Some(0x66) => 
                       { instr.operand_size_prefix = OperandSizePrefixBehavior::Always; }
                    Some(0x67) => { instr.address_size_prefix = Some(true); },
                    Some(0xF2) => { instr.f2_prefix = PrefixBehavior::Always; },
                    Some(0xF3) => { instr.f3_prefix = PrefixBehavior::Always; },
                    Some(0x0F) => { instr.two_byte_opcode = true; }, 
                    None => {},
                    _ => panic!("Invalid prefix: {:?}.", prefix)
                }
                match map {
                    OpcodeMap::M0f => { instr.two_byte_opcode = true; },
                    OpcodeMap::M0f38 => {
                        instr.two_byte_opcode = true;
                        instr.primary_opcode = 0x38;
                    },
                    OpcodeMap::M0f3a => {
                        instr.two_byte_opcode = true;
                        instr.primary_opcode = 0x3A;
                    },
                }
            },
            OpcodeToken::Rex => { 
                instr.composite_prefix = Some(CompositePrefix::Rex { size_64: None });
            },
            OpcodeToken::RexW => {
                instr.composite_prefix = Some(CompositePrefix::Rex { size_64: Some(true) });
            },
            OpcodeToken::Byte(b) => match b {
                0x66 if !instr.two_byte_opcode => { instr.operand_size_prefix = OperandSizePrefixBehavior::Always; },
                0x67 if !instr.two_byte_opcode => { instr.address_size_prefix = Some(true); },
                0xF2 | 0xF3 if match instr.composite_prefix {
                    Some(CompositePrefix::Vex { .. }) |
                    Some(CompositePrefix::Evex { .. }) => false,
                    _ => true
                } && !instr.two_byte_opcode => { if b == 0xF2 { instr.f2_prefix = PrefixBehavior::Always }
                    else { instr.f3_prefix = PrefixBehavior::Always } },
                0x0F if !instr.two_byte_opcode => { instr.two_byte_opcode = true; }, 
                _ if instr.primary_opcode == 0x38 || instr.primary_opcode == 0x3A => {
                    instr.secondary_opcode = Some(b);
                },
                _ if instr.primary_opcode == 0x9B => {
                    instr.fwait = true;
                    instr.primary_opcode = b;
                },
                _ if instr.primary_opcode != 0 => {
                    instr.fixed_mod_rm_mod = Some(b >> 6);
                    instr.fixed_mod_rm_reg = Some((b >> 3) & 0x7);
                },
                _ => { instr.primary_opcode = b; }
            },
            OpcodeToken::OpcodeExt(ext) => { instr.opcode_ext = Some(ext); },
            OpcodeToken::ModRm => { instr.has_mod_rm = true; }

            OpcodeToken::EmbeddedReg(_) |
            OpcodeToken::EmbeddedFpuReg => { has_embedded_reg = true; },

            OpcodeToken::Offset(_) |
            OpcodeToken::Imm(_) |
            OpcodeToken::Is4 |
            OpcodeToken::Vsib => {} // Opcode encoding information is in operand definitions
        }
    }

    for instr_token in instruction_parts {
        match instr_token {
            InstructionToken::Mask => { instr.allow_mask = true; },
            InstructionToken::RoundMode => { instr.allow_rounding = true; },
            InstructionToken::Sae => { instr.allow_sae = true; },
            InstructionToken::MergeMode => { instr.allow_merge_mode = true; },
            _ => {
                let enc_info = operands.pop().unwrap();
                let token_iter = if let InstructionToken::Set(items) = instr_token {
                    instr.add_operand(instr_tokens_to_operand(items.iter(), enc_info, 
                        has_embedded_reg));
                } else {
                    instr.add_operand(instr_tokens_to_operand(std::iter::once(&instr_token),
                        enc_info, has_embedded_reg));
                };
            }
        }   
    }

    instr
}

fn fixup(instr: &mut InstructionDefinition) {
    match instr.mnemonic.as_str() {
        "BSWAP" => {
            // BSWAP needs an operand size prefix in real mode
            if instr.operands[0].as_ref().map_or(false, |o| o.size == OperandSize::Dword) {
                instr.operand_size_prefix = OperandSizePrefixBehavior::RealOnly;
            }   
        },
        "CRC32" => {
            // This is sort of a weird case, as it needs REX in long mode, but is 32-bit
            if instr.valid_64 && !instr.valid_32 && 
                instr.operands[0].as_ref().map_or(false, |o| o.size == OperandSize::Dword) &&
                instr.operands[1].as_ref().map_or(false, |o| o.size == OperandSize::Byte) {
                instr.composite_prefix = Some(CompositePrefix::Rex { size_64: Some(false) });
            }
        },
        "LAR" => {
            // LAR's operand size prefix depends on the first operand, but this isn't obvious
            // TODO Is this correct?
            instr.operand_size_prefix =
                if instr.operands[1].as_ref().map_or(false, |o| o.size == OperandSize::Word)
                { OperandSizePrefixBehavior::NotReal } else { OperandSizePrefixBehavior::RealOnly };
        },
        "MOVNTI" => {
            // MOVNTI r32, r/m16 needs op size prefix in real mode, even though it's unambiguous
            if instr.operands[0].as_ref().map_or(false, |o| o.size == OperandSize::Dword) {
                instr.operand_size_prefix = OperandSizePrefixBehavior::RealOnly;
            }   
        }
        "MOVSX" |
        "MOVZX" => {
            // MOVS/ZX r32, r/m16 needs op size prefix in real mode, even though it's unambiguous
            if (instr.primary_opcode == 0xBF || instr.primary_opcode == 0xB7)
                && instr.operands[0].as_ref().map_or(false,
                |o| o.size == OperandSize::Dword) {
                instr.operand_size_prefix = OperandSizePrefixBehavior::RealOnly;
            }
        },
        // TODO Next 2?
        // "SLDT" => {
        //     instr.operand_size_prefix = OperandSizePrefixBehavior::ConditionalOnSize(0)
        // },
        // "STR" => {
        //     instr.operand_size_prefix = OperandSizePrefixBehavior::ConditionalOnSize(0)
        // },
        _ => {}
    }
}

fn mnemonic_from_instr(instr: &String) -> &str {
    if let Some(index) = instr.find(" ") {
        &instr[0..index]
    } else { &instr[..] }
}

fn instr_needs_op_size_prefix(instr: &InstructionDefinition, i: u8)
    -> bool {
    instr.operands[i as usize].as_ref().map_or(false,
        |o| o.size == OperandSize::Word || o.size == OperandSize::Dword
            || o.size == OperandSize::Far16 || o.size == OperandSize::Far32)
}

fn instr_tokens_to_operand<'a, I>(tokens: I, enc_info: Option<(OperandEncoding, OperandAccess)>,
    has_embedded_reg: bool) -> OperandDefinition where I : Iterator<Item=&'a InstructionToken> {
    let pairs: Vec<(OperandType, Option<OperandSize>)> = 
        tokens.map(instr_token_to_operand_type).collect();
    let size = pairs.iter().filter_map(|p| p.1).filter(|s| *s != OperandSize::Unsized).next()
        .unwrap_or(OperandSize::Unsized);
    let op_type = if pairs.len() == 1 {
        pairs[0].0.clone()
    } else {
        OperandType::Set(pairs.iter().map(|i| i.0.clone()).collect())
    };

    let (enc, access) = enc_info.unwrap_or_else(
        || generate_implied_encoding(&op_type, has_embedded_reg));

    OperandDefinition {
        encoding: enc,
        access: access,
        size: size,
        op_type: op_type
    }
}

fn generate_implied_encoding(op_type: &OperandType, has_embedded_reg: bool) 
    -> (OperandEncoding, OperandAccess) {
    match *op_type { // TODO Better implied operand access
        OperandType::Reg(reg_type) => {
            (OperandEncoding::ModRmRm, OperandAccess::Read)
        },
        OperandType::Mem(_) => (OperandEncoding::ModRmRm, OperandAccess::Read),
        OperandType::Imm => (OperandEncoding::Imm, OperandAccess::Read),
        OperandType::Constant => (OperandEncoding::Fixed, OperandAccess::Read),
        OperandType::Offset => (OperandEncoding::Imm, OperandAccess::Read),
        OperandType::Rel(_) => (OperandEncoding::Imm, OperandAccess::Read),
        OperandType::Mib => (OperandEncoding::ModRmRm, OperandAccess::Read),
        OperandType::Bcst(_) => (OperandEncoding::ModRmRm, OperandAccess::Read),
        OperandType::Fixed(_) => (OperandEncoding::Fixed, OperandAccess::Read),
        OperandType::Set(_) => (OperandEncoding::ModRmRm, OperandAccess::Read)
    }
}

fn instr_token_to_operand_type(token: &InstructionToken) -> (OperandType, Option<OperandSize>) {
    match *token {
        InstructionToken::Reg(reg_type, op_size)
            => (OperandType::Reg(reg_type), Some(op_size)),
        InstructionToken::Mem(op_size)
            => (OperandType::Mem(Some(op_size)), Some(op_size)),
        InstructionToken::Imm(op_size)
            => (OperandType::Imm, Some(op_size)),
        InstructionToken::Bcst(bcst_size)
            => (OperandType::Bcst(bcst_size), None),
        InstructionToken::Rel(op_size)
            => (OperandType::Rel(op_size), Some(op_size)),
        InstructionToken::Offset(op_size)
            => (OperandType::Offset, Some(op_size)),
        InstructionToken::FixedReg(reg)
            => (OperandType::Fixed(FixedOperand::Reg(reg)), Some(reg.size())),
        InstructionToken::Constant(val)
            => (OperandType::Fixed(FixedOperand::Constant(val)), Some(OperandSize::Unsized)),
        InstructionToken::Mib
            => (OperandType::Mib, Some(OperandSize::Unsized)),
        _ => panic!("Unsupported type: {:?}", *token)
    }
}

fn parse_operand_encoding_opt(operand: &str) -> Option<(OperandEncoding, OperandAccess)> {
    if operand.len() != 0 {
        Some(parse_operand_encoding(operand.as_bytes()).unwrap().1)
    } else {
        None
    }
}

named!(instruction_sep, eat_separator!(&b", "[..]));
named!(parse_token_list<Vec<Vec<InstructionToken>>>, separated_list!(instruction_sep, parse_instruction_part));
named!(parse_instruction<&[u8], (String, Vec<InstructionToken>), u32>, do_parse!(
        mnemonic: alphanumeric >> opt!(instruction_sep) >>
        tokens: opt!(complete!(parse_token_list)) >>
        (build_result(mnemonic, tokens))
    )
);

fn build_result(mnemonic: &[u8], tokens: Option<Vec<Vec<InstructionToken>>>) -> (String, Vec<InstructionToken>) {
    (String::from_utf8(mnemonic.iter().cloned().collect()).unwrap(),
        tokens.map_or(Vec::new(), |t| t.into_iter().flat_map(|v| v.into_iter()).collect())
    )
}

named!(instruction_part_separator, take_while!(|c| c == b' ' || c == b',' || c == b'{'));

named!(parse_instruction_part<Vec<InstructionToken>>, terminated!(separated_list!(instruction_part_separator,
    alt_complete!(
        tag!("z}") => { |_| InstructionToken::MergeMode } |
        alt_complete!(tag!("k1}") | tag!("k2}")) => { |_| InstructionToken::Mask } |
        tag!("er}") => { |_| InstructionToken::RoundMode } |
        tag!("sae}") => { |_| InstructionToken::Sae } |
        parse_operand
    )
), opt!(complete!(one_of!("abc")))));

named!(parse_operand<InstructionToken>, map!(get_operand_parts, parse_operand_parts_final));

named!(get_operand_parts<&[u8], Vec<InstructionToken>, u32>, separated_list!(slash_sep, parse_operand_part));

named!(slash_sep, eat_separator!(&b"/"[..]));

fn parse_operand_parts_final(parts: Vec<InstructionToken>) -> InstructionToken {
    if parts.len() == 1 { parts.into_iter().next().unwrap() }
    else { InstructionToken::Set(parts) }
}

named!(type_suffix, complete!(alt_complete!(
    tag!("fp") | 
    tag!("int") |
    tag!("dec") |
    tag!("bcd")
)));

named!(parse_operand_part<InstructionToken>, alt_complete!(
    tag!("imm8/r") => { |_| InstructionToken::Imm(OperandSize::Byte) } |
    do_parse!(tag!("rel") >> size: parse_size >> (make_sized(size, |s| InstructionToken::Rel(s))) ) |
    do_parse!(tag!("r/m") >> size: parse_size >> (make_rm(size, RegType::General)) ) |
    do_parse!(tag!("imm") >> size: parse_size >> (make_sized(size, |s| InstructionToken::Imm(s))) ) |
    do_parse!(tag!("moffs") >> size: parse_size >> (make_sized(size,
       |s| InstructionToken::Offset(s))) ) |
    tag!("Sreg") => { |_| InstructionToken::Reg(RegType::Segment, OperandSize::Word) } |
    tag!("ST(i)") => { |_| InstructionToken::Reg(RegType::Fpu, OperandSize::Tbyte) } |
    alt_complete!(tag!("ST(0)") | tag!("ST")) => { |_| InstructionToken::FixedReg(Reg::ST) } |
    do_parse!(s: one_of!("xyz") >> tag!("mm") >> opt!(complete!(digit)) >> (InstructionToken::Reg(RegType::Avx,
        avx_size(s)))) |
    tag!("m16:16") => { |_| InstructionToken::Mem(OperandSize::Far16) } |
    tag!("m16:32") => { |_| InstructionToken::Mem(OperandSize::Far32) } |
    tag!("m16:64") => { |_| InstructionToken::Mem(OperandSize::Far64) } |
    do_parse!(tag!("m") >> size: parse_size >> tag!("bcst") >>
        (make_sized(size, |s| InstructionToken::Bcst(s))) ) |
    do_parse!(tag!("m") >> size: parse_size >> 
        opt!(type_suffix) >> 
        (make_sized(size, |s| InstructionToken::Mem(s))) ) |
    do_parse!(tag!("r") >> size: parse_size >> (make_sized(size,
       |s| InstructionToken::Reg(RegType::General, s) ))) |
    tag!("AL") => { |_| InstructionToken::FixedReg(Reg::AL) } |
    tag!("AX") => { |_| InstructionToken::FixedReg(Reg::AX) } |
    tag!("EAX") => { |_| InstructionToken::FixedReg(Reg::EAX) } |
    tag!("RAX") => { |_| InstructionToken::FixedReg(Reg::RAX) } |
    tag!("CL") => { |_| InstructionToken::FixedReg(Reg::CL) } |
    tag!("DX") => { |_| InstructionToken::FixedReg(Reg::DX) } |
    tag!("CS") => { |_| InstructionToken::FixedReg(Reg::CS) } |
    tag!("DS") => { |_| InstructionToken::FixedReg(Reg::DS) } |
    tag!("ES") => { |_| InstructionToken::FixedReg(Reg::ES) } |
    tag!("FS") => { |_| InstructionToken::FixedReg(Reg::FS) } |
    tag!("GS") => { |_| InstructionToken::FixedReg(Reg::GS) } |
    tag!("SS") => { |_| InstructionToken::FixedReg(Reg::SS) } |
    tag!("<XMM0>") => { |_| InstructionToken::FixedReg(Reg::XMM0) } |
    alt_complete!(tag!("bnd1") | tag!("bnd2") | tag!("bnd"))
        => { |_| InstructionToken::Reg(RegType::Bound, OperandSize::Unsized) } |
    tag!("mib") => { |_| InstructionToken::Mib } |
    tag!("ptr16:16") => { |_| InstructionToken::Imm(OperandSize::Far16) } |
    tag!("ptr16:32") => { |_| InstructionToken::Imm(OperandSize::Far32) } |
    tag!("ptr16:64") => { |_| InstructionToken::Imm(OperandSize::Far64) } |
    do_parse!(tag!("k") >> digit >> (InstructionToken::Reg(RegType::Mask, OperandSize::Unsized))) |
    do_parse!(tag!("mm") >> opt!(complete!(digit)) >> (InstructionToken::Reg(RegType::Mmx, OperandSize::Qword))) |
    do_parse!(val: digit >> (InstructionToken::Constant(
        str::from_utf8(val).unwrap().parse::<u32>().unwrap()))) |
    tag!("reg") => { |_| InstructionToken::Reg(RegType::General, OperandSize::Unsized) } |
    alt_complete!(
        tag!("mem") |
        tag!("m") |
        tag!("vm32x") | tag!("vm32y") | tag!("vm32z") |
        tag!("vm64x") | tag!("vm64y") | tag!("vm64z")
    ) => { |_| InstructionToken::Mem(OperandSize::Unsized) } |
    alt_complete!( // TODO Could make instr defs here more specific
        tag!("CR0-CR7") |
        tag!("CR8")
    ) => { |_| InstructionToken::Reg(RegType::Control, OperandSize::Unsized) } |
    tag!("DR0-DR7") => { |_| InstructionToken::Reg(RegType::Debug, OperandSize::Unsized) }
));

named!(parse_size<OperandSize>, alt_complete!(
    alt_complete!(
        tag!("14/28byte") |
        tag!("94/108byte") |
        tag!("512byte")
    ) => { |_| OperandSize::Unsized } |
    do_parse!(digit >> tag!("&") >> digit >> (OperandSize::Unsized)) |
    tag!("80") => { |_| OperandSize::Tbyte } |
    tag!("8") => { |_| OperandSize::Byte } |
    alt!(tag!("16") | tag!("2byte")) => { |_| OperandSize::Word } |
    tag!("32") => { |_| OperandSize::Dword } |
    tag!("64") => { |_| OperandSize::Qword } |
    tag!("128") => { |_| OperandSize::Xmmword } |
    tag!("256") => { |_| OperandSize::Ymmword } |
    tag!("512") => { |_| OperandSize::Zmmword } |
    tag!("ptr16:16") => { |_| OperandSize::Far16 } |
    tag!("ptr16:32") => { |_| OperandSize::Far32 } |
    tag!("ptr16:64") => { |_| OperandSize::Far64 }
));

fn avx_size(s: char) -> OperandSize {
    match s {
        'x' => OperandSize::Xmmword,
        'y' => OperandSize::Ymmword,
        'z' => OperandSize::Zmmword,
        _ => panic!("Invalid AVX register: {}", s)
    }
}

fn make_rm(size: OperandSize, reg_type: RegType) -> InstructionToken {
    let vec = vec![InstructionToken::Reg(reg_type, size), InstructionToken::Mem(size)];
    InstructionToken::Set(vec)
}

fn make_sized<F>(size: OperandSize, constructor: F) -> InstructionToken
    where F: Fn(OperandSize) -> InstructionToken {
    constructor(size)
}
    
named!(parse_opcode<Vec<OpcodeToken>>, many1!(parse_opcode_token));

named!(parse_opcode_token<OpcodeToken>, ws!(alt_complete!(
        tag!("NP") => { |_| OpcodeToken::NoPrefix } |
        alt_complete!(
            tag_no_case!("REX.W + ") |
            tag_no_case!("REX.W" )
        ) => { |_| OpcodeToken::RexW } |
        alt_complete!(
            tag!("REX.R + ") | // TODO?
            tag!("REX + ") |
            tag!("REX")
        ) => { |_| OpcodeToken::Rex } |
        alt_complete!(tag!("ib") | tag!("imm8")) => { |_| OpcodeToken::Imm(OperandSize::Byte) } |
        tag!("iw") => { |_| OpcodeToken::Imm(OperandSize::Word) } |
        tag!("id") => { |_| OpcodeToken::Imm(OperandSize::Dword) } |
        tag!("io") => { |_| OpcodeToken::Imm(OperandSize::Qword) } |
        alt_complete!(tag!("/r") | tag!("m64") | tag!("m128")) => { |_| OpcodeToken::ModRm } |
        tag!("cb") => { |_| OpcodeToken::Offset(OperandSize::Byte) } |
        tag!("cw") => { |_| OpcodeToken::Offset(OperandSize::Word) } |
        tag!("cd") => { |_| OpcodeToken::Offset(OperandSize::Dword) } |
        tag!("cp") => { |_| OpcodeToken::Offset(OperandSize::Fword) } |
        tag!("co") => { |_| OpcodeToken::Offset(OperandSize::Qword) } |
        tag!("ct") => { |_| OpcodeToken::Offset(OperandSize::Tbyte) } |
        tag!("+rb") => { |_| OpcodeToken::EmbeddedReg(OperandSize::Byte) } |
        tag!("+rw") => { |_| OpcodeToken::EmbeddedReg(OperandSize::Word) } |
        tag!("+rd") => { |_| OpcodeToken::EmbeddedReg(OperandSize::Dword) } |
        tag!("+ro") => { |_| OpcodeToken::EmbeddedReg(OperandSize::Qword) } |
        tag!("+i") => { |_| OpcodeToken::EmbeddedFpuReg } |
        do_parse!(
            tag!("/") >>
            ext: digit >>
            (OpcodeToken::OpcodeExt(u8::from_str_radix(str::from_utf8(ext).unwrap(), 16)
                .unwrap()))
        ) |
        tag!("/is4") => { |_| OpcodeToken::Is4 } |
        tag!("/vsib") => { |_| OpcodeToken::Vsib } |
        do_parse!(
            tag!("VEX") >>
            op_behavior: opt!(alt_complete!(
                tag!(".NDS") => { |_| VexOperandBehavior::Nds } |
                tag!(".NDD") => { |_| VexOperandBehavior::Ndd } |
                tag!(".DDS") => { |_| VexOperandBehavior::Dds }
            )) >>
            vec_size: alt_complete!(
                tag!(".128") => { |_| Some(OperandSize::Xmmword) } |
                tag!(".L0") => { |_| Some(OperandSize::Xmmword) } |
                tag!(".256") => { |_| Some(OperandSize::Ymmword) } |
                tag!(".L1") => { |_| Some(OperandSize::Ymmword) } |
                alt_complete!(tag!(".LIG") | tag!(".LZ")) => { |_| None }
            ) >>
            prefix: opt!(alt_complete!(
                tag!(".66") => { |_| 0x66u8 } |
                tag!(".F2") => { |_| 0xF2u8 } |
                tag!(".F3") => { |_| 0xF3u8 }
            )) >>
            opcode_map: alt_complete!(
                tag!(".0F3A") => { |_| OpcodeMap::M0f3a } |
                tag!(".0F38") => { |_| OpcodeMap::M0f38 } |
                tag!(".0F") => { |_| OpcodeMap::M0f }
            ) >>
            w: map!(complete!(opt!(alt_complete!(
                tag!(".W0") => { |_| Some(false) } |
                tag!(".W1") => { |_| Some(true) } |
                tag!(".WIG") => { |_| None }
            ))), |v| v.and_then(|i| i)) >>
            (OpcodeToken::Vex(op_behavior, vec_size, prefix, opcode_map, w))
        ) |
        do_parse!(
            tag!("EVEX") >>
            op_behavior: opt!(complete!(alt_complete!(
                tag!(".NDS") => { |_| VexOperandBehavior::Nds } |
                tag!(".NDD") => { |_| VexOperandBehavior::Ndd } |
                tag!(".DDS") => { |_| VexOperandBehavior::Dds }
            ))) >>
            vec_size: alt_complete!(
                tag!(".128") => { |_| Some(OperandSize::Xmmword) } |
                tag!(".256") => { |_| Some(OperandSize::Ymmword) } |
                tag!(".512") => { |_| Some(OperandSize::Zmmword) } |
                tag!(".LIG") => { |_| None }
            ) >>
            prefix: opt!(alt_complete!(
                tag!(".66") => { |_| 0x66u8 } |
                tag!(".F2") => { |_| 0xF2u8 } |
                tag!(".F3") => { |_| 0xF3u8 }
            )) >>
            opcode_map: alt_complete!(
                tag!(".0F3A") => { |_| OpcodeMap::M0f3a } |
                tag!(".0F38") => { |_| OpcodeMap::M0f38 } |
                tag!(".0F") => { |_| OpcodeMap::M0f }
            ) >>
            w: map!(complete!(opt!(alt_complete!(
                tag!(".W0") => { |_| Some(false) } |
                tag!(".W1") => { |_| Some(true) } |
                tag!(".WIG") => { |_| None }
            ))), |v| v.and_then(|i| i)) >>
            (OpcodeToken::Evex(op_behavior, vec_size, prefix, opcode_map, w))
        ) |
        alphanumeric => { |val| {
            return OpcodeToken::Byte(u8::from_str_radix(str::from_utf8(val).unwrap(), 16).unwrap());
        } }
    ))
);

named!(parse_operand_encoding_type<OperandEncoding>, alt_complete!(
        alt_complete!(tag!("VEX.vvvv") | tag!("EVEX.vvvv") | tag!("vvvv"))
            => { |_| OperandEncoding::Vex } |
        tag!("ModRM:reg") => { |_| OperandEncoding::ModRmReg } |
        tag!("ModRM:r/m") => { |_| OperandEncoding::ModRmRm } |
        tag!("ModRM:rm") => { |_| OperandEncoding::ModRmRm } |
        alt_complete!(
            tag_no_case!("Imm8") |
            tag_no_case!("Imm16")
        ) => { |_| OperandEncoding::Imm } |
        alt_complete!(
            tag_no_case!("implicit XMM0") | 
            tag!("<XMM0>") |
            tag!("AL/AX/EAX/RAX") |
            tag!("AX/EAX/RAX") |
            tag!("RDX/EDX") |
            digit |
            tag!("CL")
        ) => { |_| OperandEncoding::Fixed } |
        do_parse!(
            tag!("opcode") >>
            many1!(one_of!(" ")) >>
            tag!("+") >>
            size: complete!(alt_complete!(
                tag!("rb") => { |_| OperandSize::Byte } |
                tag!("rw") => { |_| OperandSize::Word } |
                tag!("rd") => { |_| OperandSize::Dword } |
                tag!("ro") => { |_| OperandSize::Qword }
            )) >>
            (OperandEncoding::OpcodeAddend)
        ) |
        tag!("iw") => { |_| OperandEncoding::Imm } |
        tag!("Moffs") => { |_| OperandEncoding::Offset } |
        alt_complete!(
            tag!("SIB.base") |
            tag!("BaseReg")
        ) => { |_| OperandEncoding::Mib } |
        tag!("Mem") => { |_| unimplemented!() } | // TODO?
        tag!("NA") => { |_| OperandEncoding::Fixed }
));

named!(parse_operand_access<OperandAccess>, complete!(alt_complete!(
        tag!("(r)") => { |_| OperandAccess::Read } |
        tag!("(w)") => { |_| OperandAccess::Write } |
        tag!("(r, w)") => { |_| OperandAccess::ReadWrite } |
        tag!("(w, ModRM:[7:6] must not be 11b)") => { |_| OperandAccess::ReadWrite }
)));

named!(parse_operand_encoding<(OperandEncoding, OperandAccess)>, do_parse!(
    enc: parse_operand_encoding_type >>
    eat_separator!(" ") >>
    access: opt!(parse_operand_access) >>
    ((enc, access.unwrap_or(OperandAccess::Read)))
));

impl OperandSize {
    fn from_bytes(bytes: &[u8]) -> OperandSize {
        match str::from_utf8(bytes).unwrap() {
            "8" => OperandSize::Byte,
            "16" | "2byte" => OperandSize::Word,
            "32" => OperandSize::Dword,
            "64" => OperandSize::Qword,
            "80" => OperandSize::Tbyte,
            "128" => OperandSize::Xmmword,
            "256" => OperandSize::Ymmword,
            "512" => OperandSize::Zmmword,
            s @ _ => panic!("Bad operand size: {}", s)
        }
    }
}

#[derive(Debug)]
enum InstructionToken {
    Reg(RegType, OperandSize),
    Mem(OperandSize),
    Imm(OperandSize),
    Bcst(OperandSize),
    Mask,
    RoundMode,
    Sae,
    MergeMode,
    Constant(u32),
    Rel(OperandSize),
    Offset(OperandSize),
    Mib,
    OpcodeAddend,
    FixedReg(Reg),
    Set(Vec<InstructionToken>)
}

#[derive(Debug)]
enum OpcodeToken {
    NoPrefix,
    Byte(u8),
    Imm(OperandSize),
    Rex,
    RexW,
    OpcodeExt(u8),
    ModRm,
    Offset(OperandSize),
    EmbeddedReg(OperandSize),
    EmbeddedFpuReg,
    Is4,
    Vsib,
    Vex(Option<VexOperandBehavior>, Option<OperandSize>, Option<u8>, OpcodeMap, Option<bool>),
    Evex(Option<VexOperandBehavior>, Option<OperandSize>, Option<u8>, OpcodeMap, Option<bool>)
}

#[derive(Copy, Clone, Debug)]
enum OpcodeMap {
    M0f,
    M0f3a,
    M0f38
}

impl Debug for InstructionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, r#"InstructionDefinition: {{
    mnemonic: {:?},
    allow_prefix: {:?},
    operand_size_prefix: {:?},
    address_size_prefix: {:?},
    f2_prefix: PrefixBehavior::{:?},
    f3_prefix: PrefixBehavior::{:?},
    composite_prefix: {:?},
    fwait: {:?},
    two_byte_opcode: {:?},
    primary_opcode: {:?},
    secondary_opcode: {:?},
    opcode_ext: {:?},
    has_mod_rm: {:?},
    fixed_mod_rm_mod: {:?},
    fixed_mod_rm_reg: {:?},
    allow_mask: {:?},
    allow_merge_mode: {:?},
    allow_rounding: {:?},
    allow_sae: {:?},
    operands: {:?},
    valid_64: {:?},
    valid_32: {:?},
    valid_16: {:?},
    desc: {:?}
}}"#, 
        self.mnemonic,
        self.allow_prefix,
        self.operand_size_prefix,
        self.address_size_prefix,
        self.f2_prefix,
        self.f3_prefix,
        self.composite_prefix,
        self.fwait,
        self.two_byte_opcode,
        self.primary_opcode,
        self.secondary_opcode,
        self.opcode_ext,
        self.has_mod_rm,
        self.fixed_mod_rm_mod,
        self.fixed_mod_rm_reg,
        self.allow_mask,
        self.allow_merge_mode,
        self.allow_rounding,
        self.allow_sae,
        self.operands,
        self.valid_64,
        self.valid_32,
        self.valid_16,
        self.desc
        )
    }
}

impl Debug for OperandSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OperandSize::{}",
            match *self {
                OperandSize::Byte => "Byte",
                OperandSize::Word => "Word",
                OperandSize::Dword => "Dword",
                OperandSize::Fword => "Fword",
                OperandSize::Qword => "Qword",
                OperandSize::Tbyte => "Tbyte",
                OperandSize::Xmmword => "Xmmword",
                OperandSize::Ymmword => "Ymmword",
                OperandSize::Zmmword => "Zmmword",
                OperandSize::Far16 => "Far16",
                OperandSize::Far32 => "Far32",
                OperandSize::Far64 => "Far64",
                OperandSize::Unsized => "Unsized",
            })
    }
}
