use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::str;
use std::vec;
 
use nom::*;
use rand;
use rand::Rng;

use instruction_def::CompositePrefix;
use instruction_def::FixedOperand;
use instruction_def::InstructionDefinition;
use instruction_def::OperandAccess;
use instruction_def::OperandDefinition;
use instruction_def::OperandEncoding;
use instruction_def::OperandType;
use instruction_def::RegType;
use instruction_def::VexOperandBehavior;
use instruction::{BroadcastMode, Instruction, MaskReg, MergeMode, Reg, RegScale, RoundingMode, SegmentReg};
use operand::{Operand, OperandSize};

static SKIP_MNEMONICS: [&'static str; 10] = [ // TODO
    "CALL",
    "JMP", // Gnu assember won't use 16:16 in protected mode
    "LAR", // Gnu assember puts a REX.W. Not sure if it matters?
    "MOV", // Getting assembler to use relative movs is hard?
    "MOVQ", // Instruction ambiguity?
    "PUSH", // Gnu as doesn't use shortest form, auto promotes to word size?
    "SLDT", // Gnu as doesn't emit 66 for memory operands, but does for registers?
    "SMSW", // Gnu as doesn't emit 66 for memory operands, but does for registers?
    "STR", // Gnu as doesn't emit 66 for memory operands, but does for registers?
    "VMOVQ", // Interchangable forms when source is mem64
];

pub fn emit_tests(instr: &InstructionDefinition, output_dir: &str, test_count: &mut HashMap<String, u32>)
    -> io::Result<()> {
    if SKIP_MNEMONICS.iter().find(|s| instr.mnemonic.as_str() == **s).is_some() {
        return Ok(());
    }

    if instr.valid_16 { emit_tests_helper(instr, OperandSize::Word, output_dir, test_count)?; }
    if instr.valid_32 { emit_tests_helper(instr, OperandSize::Dword, output_dir, test_count)?; }
    if instr.valid_64 { emit_tests_helper(instr, OperandSize::Qword, output_dir, test_count)?; }

    Ok(())
}

pub fn emit_tests_helper(instr: &InstructionDefinition, addr_size: OperandSize, output_dir: &str,
    test_count: &mut HashMap<String, u32>) -> io::Result<()> {
    if should_skip_instr(instr) { return Ok(()); }

    let test_instrs = build_test_instructions(instr, addr_size);

    let enc_result: io::Result<Vec<Vec<u8>>> = 
        test_instrs.iter().map(|i| encode(i, instr, addr_size)).collect();
    let bytes = enc_result?;
    let mut writer = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}/{}", output_dir, instr.mnemonic.to_lowercase()))
        .unwrap();

    test_instrs.iter().zip(bytes.iter()).fold(Ok(()),
        |res, (i, b)| res.and(write_test(i, b, addr_size, &mut writer, test_count)))
}

fn should_skip_instr(instr: &InstructionDefinition) -> bool {
    (instr.mnemonic.as_str() == "MOV" && instr.operands.iter().any(
        |o| o.as_ref().map(|def| if let OperandType::Reg(RegType::Segment) = def.op_type {
            true } else { false }).unwrap_or(false))) // Skip segment reg movs
    || instr.mnemonic.find("PEXTR").is_some()
    || instr.mnemonic.find("PTWRITE").is_some() // GAS doesn't recognize
    || instr.mnemonic.find("RDPID").is_some() // GAS doesn't recognize
    || instr.mnemonic.find("SQRTPD").is_some() // TODO Size mismatch?
    || instr.mnemonic.find("UD1").is_some() // GAS doesn't recognize UD1 with operands?
    || instr.mnemonic.find("VGATHER").is_some() // TODO VSIB
    || instr.mnemonic.find("VPGATHER").is_some() // TODO VSIB
    // TODO Generic register sizes (GAS doesn't like)
    || instr.mnemonic.find("VPBROADCAST").is_some() 
    || instr.mnemonic.find("VPMOVQD").is_some()  // TODO Unknown?
    || instr.mnemonic.find("VPSCATTER").is_some() // TODO Require mask
    || instr.mnemonic.find("VSCATTER").is_some() // TODO Require mask
    || instr.mnemonic.find("XSAVEOPT64").is_some() // TODO Intel docs wrong? Only valid 64?
}

fn make_operand_combinations(instr: &InstructionDefinition) -> Vec<Vec<OperandDefinition>> {
    let set_parts = instr.operands.iter().by_ref().filter_map(
            |maybe_op| maybe_op.as_ref().and_then(
                |op| if let OperandType::Set(ref items) = op.op_type {
                    if instr.mnemonic.find("CVT").is_none() {
                        Some(items.clone())
                    } else {
                        Some(items.iter().filter(|i| if let OperandType::Bcst(_) = **i { false }
                            else { true }).cloned().collect())
                    }
                } else { None }
            )
        ).next();
    if let Some(parts) = set_parts { 
        parts.iter().map(|part| instr.operands.iter().filter_map(
            |maybe_op| maybe_op.as_ref().map(|op| if let OperandType::Set(_) = op.op_type {
                OperandDefinition {
                    encoding: op.encoding,
                    access: op.access,
                    size: op.size,
                    op_type: part.clone()
                }
            } else { op.clone() }
        )).collect()).collect()
    } else {
        vec![instr.operands.iter().filter_map(|x| x.as_ref()).cloned().collect()]
    }
}

fn build_test_instructions(def: &InstructionDefinition, addr_size: OperandSize) -> Vec<Instruction> {
    let op_combinations = make_operand_combinations(def);
    op_combinations.into_iter().filter(filter_op_combination)
        .map(|op_c| build_test_instruction(def, op_c, addr_size)).collect()
}

fn filter_op_combination(ops: &Vec<OperandDefinition>) -> bool {
    ops.iter().by_ref().any(|o| match o.op_type {
        OperandType::Rel(_) => false,
        _ => true
    })
}

fn build_test_instruction(def: &InstructionDefinition, op_defs: Vec<OperandDefinition>,
    addr_size: OperandSize) -> Instruction {

    let mut instr = Instruction {
        mnemonic: def.mnemonic.clone(),
        .. Default::default()
    };

    let first_op_not_mem = op_defs.iter().next().map(|o| !o.op_type.is_mem()).unwrap_or(true);
    if def.allow_mask && first_op_not_mem { instr.mask = Some(random_mask()); }
    if def.allow_merge_mode && first_op_not_mem { instr.merge_mode = Some(MergeMode::Zero) }

    if op_defs.iter().all(|d| !d.op_type.is_mem()) {
        if def.allow_rounding & op_defs.iter().all(
            |op_def| if let OperandType::Reg(_) = op_def.op_type { true } else { false })
            { instr.rounding_mode = Some(random_rounding_mode()); }
        else if def.allow_sae { instr.sae = true; }
    }

    let broadcast_size = op_defs.iter().filter_map(
        |op_def| if let OperandType::Bcst(bcst_size) = 
            op_def.op_type { Some(bcst_size) } else { None }).next();
        
    if let Some(bcst_size) = broadcast_size {
        let base_size = op_defs.iter().map(|op_def| op_def.size).max().expect("No size.");
        instr.broadcast = Some(match base_size.bits() / bcst_size.bits() {
            2 => BroadcastMode::Broadcast1To2,
            4 => BroadcastMode::Broadcast1To4,
            8 => BroadcastMode::Broadcast1To8,
            16 => BroadcastMode::Broadcast1To16,
            _ => panic!("Invalid broadcast: {:?}/{:?}.", base_size, bcst_size)
        });
    }

    let ops = op_defs.iter().map(|op_def| build_test_operand(&mut instr, def, op_def, addr_size))
        .collect::<Vec<Operand>>();

    let mut ops_iter = ops.into_iter();
    instr.operand1 = ops_iter.next();
    instr.operand2 = ops_iter.next();
    instr.operand3 = ops_iter.next();
    instr.operand4 = ops_iter.next();
    instr
}

fn build_test_operand(instr: &mut Instruction, instr_def: &InstructionDefinition,
    def: &OperandDefinition, addr_size: OperandSize) -> Operand {
    match def.op_type {
        OperandType::Reg(reg_type) =>
            Operand::Direct(random_reg(reg_type, def.size, addr_size, instr_def)),
        OperandType::Mem(size) => random_mem(size.unwrap_or(def.size), addr_size),
        OperandType::Imm => random_imm(def.size),
        OperandType::Offset => Operand::Offset(rand_value_of_size(def.size), Some(def.size), None),
        OperandType::Rel(op_size) => random_imm(op_size), // TODO Is this correct?
        OperandType::Mib => random_mib(def.size, addr_size),
        OperandType::Bcst(bcst_size) => random_mem(bcst_size, addr_size),
        OperandType::Fixed(fixed_op) => random_fixed(fixed_op),
        OperandType::Constant => unimplemented!(), // TODO What is this?
        _ => unreachable!() // Set(_) should be split apart already
    }
}

fn write_test<W: Write>(instr: &Instruction, encoded: &[u8], addr_size: OperandSize,
    writer: &mut W, test_count: &mut HashMap<String, u32>) -> io::Result<()> {
    let test_num = test_count.entry(instr.mnemonic.clone()).or_insert(0);
    *test_num += 1;

    write!(writer, "#[test]\nfn {}_{}() {{\n    run_test(&{:?}, &{:?}, {:?})\n}}\n\n",
        instr.mnemonic.to_lowercase(), test_num, instr, encoded, addr_size)
}

fn encode(instr: &Instruction, def: &InstructionDefinition, addr_size: OperandSize)
    -> io::Result<Vec<u8>> {
    // Write instruction to file
    let mut test_file = File::create("test.s")?;
    write!(test_file, ".intel_syntax noprefix\n")?;
    write!(test_file, ".code{}\n", match addr_size {
        OperandSize::Word => "16",
        OperandSize::Dword => "32",
        OperandSize::Qword => "64",
        _ => panic!("Invalid addressing size.")
    })?;
    write_instruction(instr, def, &mut test_file)?;
    write!(test_file, "\n")?;

    // Run assembler
    let as_result = Command::new("as")
        .args(&["test.s", "-o", "test.out"])
        .spawn()?
        .wait()?;
    if !as_result.success() {
        panic!("Encoding failed.")
        // return Err(Error::new(ErrorKind::Other, "As process failed."));
    }

    // Run objcopy to get bytes
    let objdump_output = Command::new("objcopy")
        .args(&["-O", "binary", "--only-section=.text", "test.out", "test.bin"])
        .spawn()?
        .wait()?;
    
    // Read bytes
    let mut bytes = Vec::new();
    File::open("test.bin")?.read_to_end(&mut bytes)?;

    // print!("{:?}: ", addr_size);
    // write_instruction(instr, def, &mut io::stdout());
    // println!(" - {:?}", bytes);

    Ok(bytes)
}

fn random_of<T: Clone>(values: &[T]) -> T { 
    let index = rand::random::<usize>() % values.len();
    values[index].clone()
}

fn random_of_func<T>(values: &[&Fn() -> T]) -> T { 
    let index = rand::random::<usize>() % values.len();
    values[index]()
}

fn write_instruction<W: Write>(instr: &Instruction, def: &InstructionDefinition, f: &mut W)
    -> io::Result<()> {
    if instr.lock { write!(f, "lock ")?; }

    write!(f, "{}", instr.mnemonic)?;

    let operands = instr.operands(); 
    let mut filtered_operands = operands.iter().zip(def.operands.iter())
        .filter_map(|(op, def)| op.as_ref().and_then(|o| def.as_ref().map(|d| (o, d))));
    let maybe_first = filtered_operands.next();
    if let Some((first, _)) = maybe_first {
        write!(f, " ")?;
        write_operand(first, def, f)?;
        if let Some(mask) = instr.mask { write!(f, " {{{:?}}}", mask)?; }
        if let Some(MergeMode::Zero) = instr.merge_mode { write!(f, " {{z}}")?; }
    }

    let mut emitted_post = false;

    for (op, op_def) in filtered_operands {
        if let OperandType::Imm = op_def.op_type {
            emitted_post = true;

            if let Some(mode) = instr.rounding_mode { write!(f, ", {{{}-sae}}", match mode {
                RoundingMode::Nearest => "rn",
                RoundingMode::Down => "rd",
                RoundingMode::Up => "ru",
                RoundingMode::Zero => "rz"
            })?; } else if instr.sae { write!(f, ", {{sae}}")?; }
        }

        write!(f, ", ")?;
        write_operand(op, def, f)?;

        let is_bcst = match op_def.op_type {
            OperandType::Bcst(_) => true,
            OperandType::Set(ref items) => items.iter().any(
                |i| if let OperandType::Bcst(_) = *i { true } else { false }),
            _ => false
        };

        if is_bcst {
            if let Some(bcst) = instr.broadcast { write!(f, " {{{}}}", match bcst {
                BroadcastMode::Broadcast1To2 => "1to2",
                BroadcastMode::Broadcast1To4 => "1to4",
                BroadcastMode::Broadcast1To8 => "1to8",
                BroadcastMode::Broadcast1To16 => "1to16",
            })?; }
        }
    }

    if !emitted_post {
        if let Some(mode) = instr.rounding_mode { write!(f, ", {{{}-sae}}", match mode {
            RoundingMode::Nearest => "rn",
            RoundingMode::Down => "rd",
            RoundingMode::Up => "ru",
            RoundingMode::Zero => "rz"
        })?; } else if instr.sae { write!(f, ", {{sae}}")?; }
    }

    Ok(())
}

fn write_operand<W: Write>(op: &Operand, instr_def: &InstructionDefinition, f: &mut W)
    -> io::Result<()> { 
    match *op {
        Operand::Direct(reg) => write!(f, "{}", reg),
        Operand::Indirect(reg, size, seg) => 
            write_indirect(f, Some(reg), None, None, None, size, seg, instr_def),
        Operand::IndirectDisplaced(reg, dsp, size, seg) =>
            write_indirect(f, Some(reg), None, None, Some(dsp), size, seg, instr_def),
        Operand::IndirectScaledIndexed(base, index, scale, size, seg) => 
            write_indirect(f, Some(base), Some(index), Some(scale), None, size, seg, instr_def),
        Operand::IndirectScaledIndexedDisplaced(base, index, scale, dsp, size, seg) =>
            write_indirect(f, Some(base), Some(index), Some(scale), Some(dsp), size, seg,
            instr_def),
        Operand::IndirectScaledDisplaced(reg, scale, dsp, size, seg) =>
            write_indirect(f, Some(reg), None, Some(scale), Some(dsp), size, seg, instr_def),
        Operand::Memory(addr, size, seg) |
        Operand::Offset(addr, size, seg) => size_seg_helper(f, size, seg, |fmt| write!(fmt, "[{}]", addr)), // TODO Is this correct?
        Operand::Literal8(v) => write!(f, "0x{:X}", v),
        Operand::Literal16(v) => write!(f, "0x{:X}", v),
        Operand::Literal32(v) => write!(f, "0x{:X}", v),
        Operand::Literal64(v) => write!(f, "0x{:X}", v),
        Operand::MemoryAndSegment16(seg, addr) => write!(f, "0x{:X}:0x{:X}", seg, addr),
        Operand::MemoryAndSegment32(seg, addr) => write!(f, "0x{:X}:0x{:X}", seg, addr),
    }
}

fn write_indirect<W: Write>(f: &mut W, base: Option<Reg>, index: Option<Reg>,
    scale: Option<RegScale>, dsp: Option<u64>, size: Option<OperandSize>,
    seg: Option<SegmentReg>, def: &InstructionDefinition) -> io::Result<()> {

    if should_write_size(def) {
        size_seg_helper(f, size, seg, |_| Ok(()))?;
    }

    write!(f, "[")?;
    if let Some(b) = base { write!(f, "{}", b)?; }
    if let Some(i) = index { write!(f, "+{}", i)?; }
    if let Some(s) = scale { write_reg_scale(&s, f)?; }
    if let Some(d) = dsp { write!(f, "+{}", d)?; }
    write!(f, "]")
}

fn should_write_size(def: &InstructionDefinition) -> bool {
    !def.operands.iter().by_ref().any(
        |op| if let Some(&OperandType::Reg(RegType::Bound)) = op.as_ref().map(|o| &o.op_type)
            { true } else { false })
     && def.mnemonic.find("CVTDQ2PD").is_none()
     && def.mnemonic.as_str() != "INVPCID"
     && def.mnemonic.as_str() != "LAR"
     && def.mnemonic.as_str() != "LSL"
}

fn write_reg_scale<W: Write>(reg_scale: &RegScale, f: &mut W) -> io::Result<()> {
    write!(f, "{}", match *reg_scale {
        RegScale::One => "", // TODO Better solution for outputting in real mode
        RegScale::Two => "*2",
        RegScale::Four => "*4",
        RegScale::Eight => "*8",
    })
}

fn size_seg_helper<F, W: Write>(f: &mut W, size: Option<OperandSize>, seg: Option<SegmentReg>,
    action: F) -> io::Result<()> where F: Fn(&mut W) -> io::Result<()> {
    write_size(f, size)?;
    write_seg(f, seg)?;
    action(f)
}

fn write_seg<W: Write>(f: &mut W, seg: Option<SegmentReg>) -> io::Result<()> {
    if let Some(s) = seg {
        write!(f, "{}", match s {
            SegmentReg::CS => "CS:",
            SegmentReg::DS => "DS:",
            SegmentReg::ES => "ES:",
            SegmentReg::FS => "FS:",
            SegmentReg::GS => "GS:",
            SegmentReg::SS => "SS:",
            _ => return Ok(())
        })?;
    }

    Ok(())
}

fn write_size<W: Write>(f: &mut W, size: Option<OperandSize>) -> io::Result<()> {
    if let Some(s) = size {
        write!(f, "{} PTR ", match s {
            OperandSize::Byte => "BYTE",
            OperandSize::Word => "WORD",
            OperandSize::Dword => "DWORD",
            OperandSize::Fword => "FWORD",
            OperandSize::Qword => "QWORD",
            OperandSize::Tbyte => "TBYTE",
            OperandSize::Xmmword => "XMMWORD",
            OperandSize::Ymmword => "YMMWORD",
            OperandSize::Zmmword => "ZMMWORD",
            _ => return Ok(())
        })?;
    }

    Ok(())
}

fn random_reg_8() -> Reg { random_of(&[Reg::BL, Reg::CL, Reg::DL]) }
fn random_reg_16() -> Reg
    { random_of(&[Reg::BX, Reg::CX, Reg::DX, Reg::SI, Reg::DI, Reg::SP, Reg::BP]) }
fn random_reg_32() -> Reg
    { random_of(&[Reg::EBX, Reg::ECX, Reg::EDX, Reg::ESI, Reg::EDI, Reg::ESP, Reg::EBP]) }
fn random_reg_64() -> Reg
    { random_of(&[Reg::RBX, Reg::RCX, Reg::RDX, Reg::RSI, Reg::RDI, Reg::RSP, Reg::RBP]) }

fn random_reg_of_size(size: OperandSize) -> Reg {
    match size {
        OperandSize::Byte => random_reg_8(),
        OperandSize::Word => random_reg_16(),
        OperandSize::Dword => random_reg_32(),
        OperandSize::Qword => random_reg_64(),
        _ => panic!("Invalid general register size: {:?}.", size)
    }
}

fn random_reg_of_size_no_stack(size: OperandSize) -> Reg {
    match size {
        OperandSize::Byte => random_reg_8(),
        OperandSize::Word => random_reg_16_no_stack(),
        OperandSize::Dword => random_reg_32_no_stack(),
        OperandSize::Qword => random_reg_64_no_stack(),
        _ => panic!("Invalid general register size: {:?}.", size)
    }
}

fn random_reg_16_no_stack() -> Reg
    { random_of(&[Reg::AX, Reg::BX, Reg::CX, Reg::DX, Reg::SI, Reg::DI]) }
fn random_reg_32_no_stack() -> Reg
    { random_of(&[Reg::EAX, Reg::EBX, Reg::ECX, Reg::EDX, Reg::ESI, Reg::EDI]) }
fn random_reg_64_no_stack() -> Reg
    { random_of(&[Reg::RAX, Reg::RBX, Reg::RCX, Reg::RDX, Reg::RSI, Reg::RDI]) }

fn random_fpu_reg() -> Reg
    { random_of(&[Reg::ST1, Reg::ST2, Reg::ST3, Reg::ST4, Reg::ST5, Reg::ST6, Reg::ST7]) }

fn random_mmx_reg() -> Reg
    { random_of(&[Reg::MM0, Reg::MM1, Reg::MM2, Reg::MM3, Reg::MM4, Reg::MM5, Reg::MM6, Reg::MM7]) }

fn random_mask_reg() -> Reg
    { random_of(&[Reg::K1, Reg::K2, Reg::K3, Reg::K4, Reg::K5, Reg::K6, Reg::K7]) }

fn random_bound_reg() -> Reg { random_of(&[Reg::BND0, Reg::BND1, Reg::BND2, Reg::BND3]) }

fn random_segment_reg() -> Reg { random_of(&[Reg::CS, Reg::DS, Reg::ES, Reg::FS, Reg::GS, Reg::SS]) }

fn random_control_reg() -> Reg { random_of(&[Reg::CR0, Reg::CR1, Reg::CR2, Reg::CR3, Reg::CR4 ]) }

fn random_debug_reg() -> Reg { random_of(&[Reg::DR0, Reg::DR1, Reg::DR2, Reg::DR3, Reg::DR4, Reg::DR5]) }

fn random_xmm_reg(use_all: bool) -> Reg { 
    if use_all { random_of(&[
        Reg::XMM0,  Reg::XMM1,  Reg::XMM2,  Reg::XMM3,  Reg::XMM4,  Reg::XMM5,  Reg::XMM6,  Reg::XMM7,
        Reg::XMM8,  Reg::XMM9,  Reg::XMM10, Reg::XMM11, Reg::XMM12, Reg::XMM13, Reg::XMM14, Reg::XMM15,
        Reg::XMM16, Reg::XMM17, Reg::XMM18, Reg::XMM19, Reg::XMM20, Reg::XMM21, Reg::XMM22, Reg::XMM23,
        Reg::XMM24, Reg::XMM25, Reg::XMM26, Reg::XMM27, Reg::XMM28, Reg::XMM29, Reg::XMM30, Reg::XMM31
    ]) } else { random_of(&[
        Reg::XMM0,  Reg::XMM1,  Reg::XMM2,  Reg::XMM3,  Reg::XMM4,  Reg::XMM5,  Reg::XMM6,  Reg::XMM7,
    ]) }
}

fn random_ymm_reg(use_all: bool) -> Reg { 
    if use_all { random_of(&[
        Reg::YMM0,  Reg::YMM1,  Reg::YMM2,  Reg::YMM3,  Reg::YMM4,  Reg::YMM5,  Reg::YMM6,  Reg::YMM7,
        Reg::YMM8,  Reg::YMM9,  Reg::YMM10, Reg::YMM11, Reg::YMM12, Reg::YMM13, Reg::YMM14, Reg::YMM15,
        Reg::YMM16, Reg::YMM17, Reg::YMM18, Reg::YMM19, Reg::YMM20, Reg::YMM21, Reg::YMM22, Reg::YMM23,
        Reg::YMM24, Reg::YMM25, Reg::YMM26, Reg::YMM27, Reg::YMM28, Reg::YMM29, Reg::YMM30, Reg::YMM31
    ]) } else { random_of(&[
        Reg::YMM0,  Reg::YMM1,  Reg::YMM2,  Reg::YMM3,  Reg::YMM4,  Reg::YMM5,  Reg::YMM6,  Reg::YMM7,
    ]) }
}

fn random_zmm_reg(use_all: bool) -> Reg { 
    if use_all { random_of(&[
        Reg::ZMM0,  Reg::ZMM1,  Reg::ZMM2,  Reg::ZMM3,  Reg::ZMM4,  Reg::ZMM5,  Reg::ZMM6,  Reg::ZMM7,
        Reg::ZMM8,  Reg::ZMM9,  Reg::ZMM10, Reg::ZMM11, Reg::ZMM12, Reg::ZMM13, Reg::ZMM14, Reg::ZMM15,
        Reg::ZMM16, Reg::ZMM17, Reg::ZMM18, Reg::ZMM19, Reg::ZMM20, Reg::ZMM21, Reg::ZMM22, Reg::ZMM23,
        Reg::ZMM24, Reg::ZMM25, Reg::ZMM26, Reg::ZMM27, Reg::ZMM28, Reg::ZMM29, Reg::ZMM30, Reg::ZMM31
    ]) } else { random_of(&[
        Reg::ZMM0,  Reg::ZMM1,  Reg::ZMM2,  Reg::ZMM3,  Reg::ZMM4,  Reg::ZMM5,  Reg::ZMM6,  Reg::ZMM7,
    ]) }
}

fn random_reg_scale() -> RegScale
    { random_of(&[RegScale::Two, RegScale::Four, RegScale::Eight]) }

fn random_reg(reg_type: RegType, size: OperandSize, addr_size: OperandSize, 
    def: &InstructionDefinition) -> Reg {
    match reg_type {
        RegType::General => {
            match size {
                OperandSize::Byte => random_reg_8(),
                OperandSize::Word => random_reg_16(),
                OperandSize::Dword => random_reg_32(),
                OperandSize::Qword => random_reg_64(),
                OperandSize::Unsized => random_reg_of_size(addr_size),
                _ => panic!("Invalid general register size: {:?}.", size)
            }
        },
        RegType::Avx => {
            let allow_all = if let Some(CompositePrefix::Evex {..}) = def.composite_prefix {
                addr_size == OperandSize::Qword } else { false };
            match size {
                OperandSize::Xmmword => random_xmm_reg(allow_all),
                OperandSize::Ymmword => random_ymm_reg(allow_all),
                OperandSize::Zmmword => random_zmm_reg(allow_all),
                _ => panic!("Invalid avx register size: {:?}.", size)
            }
        },
        RegType::Mmx => random_mmx_reg(),
        RegType::Fpu => random_fpu_reg(),
        RegType::Bound => random_bound_reg(),
        RegType::Mask => random_mask_reg(),
        RegType::Segment => random_segment_reg(),
        RegType::Control => random_control_reg(),
        RegType::Debug => random_debug_reg()
    }
}

fn random_mem(size: OperandSize, addr_size: OperandSize) -> Operand {
    if addr_size != OperandSize::Word {
        match rand::random::<u32>() % 5 { // Select addressing mode
            0 => { // Indirect - [EAX]
                Operand::Indirect(
                    random_reg_of_size_no_stack(addr_size),
                    Some(size), None)
            },
            1 => { // Indirect Displaced - [EAX+5]
                Operand::IndirectDisplaced(
                    random_reg_of_size_no_stack(addr_size),
                    (rand::random::<u32>() as u64) & 0x7FFFFFFF,
                    Some(size), None)
            },
            2 => { // Indirect Scaled Indexed - [EAX+EBX*2]
                Operand::IndirectScaledIndexed(
                    random_reg_of_size_no_stack(addr_size),
                    random_reg_of_size_no_stack(addr_size),
                    random_reg_scale(),
                    Some(size), None)
            },
            3 => { // Indirect Scaled Indexed Displaced - [EAX+EBX*2+5]
                Operand::IndirectScaledIndexedDisplaced(
                    random_reg_of_size_no_stack(addr_size),
                    random_reg_of_size_no_stack(addr_size),
                    random_reg_scale(),
                    (rand::random::<u32>() as u64) & 0x7FFFFFFF,
                    Some(size), None)

            },
            4 => { // Indirect Scaled Displaced - [EBX*2+5]
                Operand::IndirectScaledDisplaced(
                    random_reg_of_size_no_stack(addr_size),
                    random_reg_scale(),
                    (rand::random::<u32>() as u64) & 0x7FFFFFFF,
                    Some(size), None)
            },
            _ => unreachable!()
        }
    } else { // 16-bit addressing
        fn rand_16() -> u16 {
            let mut gen = rand::thread_rng();
            gen.gen_range::<u16>(u8::max_value() as u16 + 1, i16::max_value() as u16)
        }

        random_of_func(&[
            &|| Operand::IndirectScaledIndexed(Reg::BX, Reg::SI, RegScale::One, Some(size), None),
            &|| Operand::IndirectScaledIndexed(Reg::BX, Reg::DI, RegScale::One, Some(size), None),
            &|| Operand::IndirectScaledIndexed(Reg::BP, Reg::SI, RegScale::One, Some(size), None),
            &|| Operand::IndirectScaledIndexed(Reg::BP, Reg::DI, RegScale::One, Some(size), None),
            &|| Operand::Indirect(Reg::SI, Some(size), None),
            &|| Operand::Indirect(Reg::DI, Some(size), None),
            &|| Operand::Memory(rand_16() as u64, Some(size), None),
            &|| Operand::Indirect(Reg::BX, Some(size), None),

            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BX, Reg::SI, RegScale::One, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BX, Reg::DI, RegScale::One, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BP, Reg::SI, RegScale::One, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BP, Reg::DI, RegScale::One, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::SI, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::DI, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::BP, rand::random::<u8>() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::BX, rand::random::<u8>() as u64, Some(size), None),

            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BX, Reg::SI, RegScale::One, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BX, Reg::DI, RegScale::One, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BP, Reg::SI, RegScale::One, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectScaledIndexedDisplaced(
                 Reg::BP, Reg::DI, RegScale::One, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::SI, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::DI, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::BP, rand_16() as u64, Some(size), None),
            &|| Operand::IndirectDisplaced(Reg::BX, rand_16() as u64, Some(size), None),
        ])
    }
}

fn random_imm(size: OperandSize) -> Operand {
    let mut gen = rand::thread_rng();
    match size {
        OperandSize::Byte => Operand::Literal8(gen.gen_range::<u8>(0, 128)),
        OperandSize::Word => Operand::Literal16(
            gen.gen_range::<u16>(u8::max_value() as u16 + 1, i16::max_value() as u16)),
        OperandSize::Dword => Operand::Literal32(
            gen.gen_range::<u32>(u16::max_value() as u32 + 1, i32::max_value() as u32)),
        OperandSize::Qword => Operand::Literal64(
            gen.gen_range::<u64>(u32::max_value() as u64 + 1, i64::max_value() as u64)),
        OperandSize::Far16 => Operand::MemoryAndSegment16(rand::random(), rand::random()),
        OperandSize::Far32 => Operand::MemoryAndSegment32(rand::random(), rand::random()),
        _ => panic!("Invalid immediate value size: {:?}.", size)
    }
}

fn rand_value_of_size(size: OperandSize) -> u64 {
    match size {
        OperandSize::Byte => (rand::random::<u8>() as u64) & 0xF,
        OperandSize::Word => (rand::random::<u16>() as u64) & 0xFFF,
        OperandSize::Dword => (rand::random::<u32>() as u64) & 0xFFFFFF,
        OperandSize::Qword => (rand::random::<u64>() & 0xFFFFFFFFFFFFFF),
        _ => panic!("Invalid size.")
    }
}

fn random_mib(size: OperandSize, addr_size: OperandSize) -> Operand {
    Operand::IndirectScaledIndexed(
        random_reg_of_size_no_stack(addr_size),
        random_reg_of_size_no_stack(addr_size),
        RegScale::One,
        Some(size), None)
}

fn random_fixed(fixed_op: FixedOperand) -> Operand {
    match fixed_op {
        FixedOperand::Reg(reg) => Operand::Direct(reg),
        FixedOperand::Constant(c) => Operand::Literal8(c as u8)
    }
}

fn random_rounding_mode() -> RoundingMode {
    random_of(&[RoundingMode::Nearest, RoundingMode::Down, RoundingMode::Up, RoundingMode::Zero])
}

fn random_mask() -> MaskReg {
    random_of(&[MaskReg::K1, MaskReg::K2, MaskReg::K3, MaskReg::K4, MaskReg::K5, MaskReg::K6,
        MaskReg::K7])
}

named!(parse_as_output<Vec<u8>>, do_parse!(
    take_until_and_consume!("0:\t") >>
    bytes: flat_map!(
        take_until!("\t"),
        ws!(many1!(parse_u8_hex_str))
    ) >>
    (bytes)
));

named!(parse_u8_hex_str<u8>, map!(
    alphanumeric,
    |val| u8::from_str_radix(str::from_utf8(val).unwrap(), 16).unwrap()
));

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Reg::ST0 => write!(f, "ST(0)"),
            Reg::ST1 => write!(f, "ST(1)"),
            Reg::ST2 => write!(f, "ST(2)"),
            Reg::ST3 => write!(f, "ST(3)"),
            Reg::ST4 => write!(f, "ST(4)"),
            Reg::ST5 => write!(f, "ST(5)"),
            Reg::ST6 => write!(f, "ST(6)"),
            Reg::ST7 => write!(f, "ST(7)"),
            _ => write!(f, "{:?}", self)
        }
    }
}
