use std::io::Write;
use ::{Instruction, Mnemonic, Mode, Operand, OperandSize, Reg, RegScale};
use ::instruction_buffer::{ImmediateValue, InstructionBuffer};
use ::instruction_def::*;

pub struct InstructionWriter<T: Write> {
    writer: T,
    mode: Mode,
}

impl<T: Write> InstructionWriter<T> {
    pub fn new(writer: T, mode: Mode) -> InstructionWriter<T> {
        InstructionWriter {
            writer: writer,
            mode: mode,
        }
    }

    pub fn get_inner_writer_ref(&self) -> &T { &self.writer }

    pub fn write(&mut self, instr: &Instruction) -> Result<usize, InstructionEncodingError> {
        instr.encode(&mut self.writer, self.mode)
    }

    pub fn write0(&mut self, mnemonic: Mnemonic) -> Result<usize, InstructionEncodingError> {
        Instruction {
            mnemonic: mnemonic,
            .. Default::default()
        } .encode(&mut self.writer, self.mode)
    }

    pub fn write1(&mut self, mnemonic: Mnemonic, operand1: Operand) -> Result<usize, InstructionEncodingError> {
        Instruction {
            mnemonic: mnemonic,
            operand1: Some(operand1),
            .. Default::default()
        } .encode(&mut self.writer, self.mode)
    }

    pub fn write2(&mut self, mnemonic: Mnemonic, operand1: Operand, operand2: Operand) -> Result<usize, InstructionEncodingError> {
        Instruction {
            mnemonic: mnemonic,
            operand1: Some(operand1),
            operand2: Some(operand2),
            .. Default::default()
        } .encode(&mut self.writer, self.mode)
    }

    pub fn write3(&mut self, mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand) -> Result<usize, InstructionEncodingError> {
        Instruction {
            mnemonic: mnemonic,
            operand1: Some(operand1),
            operand2: Some(operand2),
            operand3: Some(operand3),
            .. Default::default()
        } .encode(&mut self.writer, self.mode)
    }

    pub fn write4(&mut self, mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand, operand4: Operand) -> Result<usize, InstructionEncodingError> {
        Instruction {
            mnemonic: mnemonic,
            operand1: Some(operand1),
            operand2: Some(operand2),
            operand3: Some(operand3),
            operand4: Some(operand4),
            .. Default::default()
        } .encode(&mut self.writer, self.mode)
    }
}

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

pub fn encode<W>(writer: &mut W, def: &InstructionDefinition, instr: &Instruction, mode: Mode) -> Result<usize, InstructionEncodingError>
    where W : Write {
    let mut buffer: InstructionBuffer = Default::default(); 

    let addr_size = get_addr_size(def, instr, mode)?;

    buffer.fwait = def.fwait;
    buffer.operand_size_prefix = get_operand_size_prefix(def.operand_size_prefix, mode);
    buffer.address_size_prefix = def.address_size_prefix.unwrap_or_else(
        || get_address_size_prefix(addr_size, mode));

    match def.f2_prefix {
        PrefixBehavior::Always => buffer.f2_prefix = true,
        PrefixBehavior::Optional => { }, // TODO
        PrefixBehavior::Never => buffer.f2_prefix = false,
    }

    match def.f3_prefix {
        PrefixBehavior::Always => buffer.f3_prefix = true,
        PrefixBehavior::Optional => { }, // TODO
        PrefixBehavior::Never => buffer.f3_prefix = false,
    }

    buffer.operand_size_64 = get_op_size_64(def);
    if def.opcode_ext.is_some() { buffer.mod_rm_reg = def.opcode_ext; }
    if def.fixed_mod_rm_mod.is_some() { buffer.mod_rm_mod = def.fixed_mod_rm_mod; }
    if def.fixed_mod_rm_reg.is_some() { buffer.mod_rm_reg = def.fixed_mod_rm_reg; }

    if let Some(pref) = def.composite_prefix {
        match pref {
            CompositePrefix::Rex { /*size_64: s*/ .. } => { buffer.operand_size_64 = true }, // TODO?
            CompositePrefix::Vex { vector_size: vs, operand_behavior: _, we: w } => {
                buffer.vex_e = w;
                buffer.vector_len = vs.map(|s| s == OperandSize::Ymmword);
                buffer.vex_l = vs.map(|s| s == OperandSize::Zmmword);
                buffer.composite_prefix = Some(::instruction_buffer::CompositePrefix::Vex);
            },
            CompositePrefix::Evex { vector_size: vs, operand_behavior: _, we: w } => {
                buffer.vex_e = w;
                buffer.vector_len = vs.map(|s| s == OperandSize::Ymmword);
                buffer.vex_l = vs.map(|s| s == OperandSize::Zmmword);
                buffer.composite_prefix = Some(::instruction_buffer::CompositePrefix::Evex);
            },
        }
    }

    buffer.is_two_byte_opcode = def.two_byte_opcode;
    buffer.primary_opcode = def.primary_opcode;
    buffer.secondary_opcode = def.secondary_opcode;

    buffer.mask_reg = instr.mask.map(|m| m.get_reg_code());
    buffer.merge_mode = instr.merge_mode;

    if instr.sae {
        buffer.vex_b = Some(true);
        buffer.vector_len = Some(false);
        buffer.vex_l = Some(false);
    }

    if let Some(_) = instr.broadcast { buffer.vex_b = Some(true); }

    if let Some(code) = instr.rounding_mode.map(|r| r.get_code()) {
        buffer.vex_b = Some(true);
        buffer.vex_l = Some(code & 0x2 != 0);
        buffer.vector_len = Some(code & 0x1 != 0);
    }

    for (maybe_op_def, op) in def.operands.iter().zip(instr.operands().iter()) {
        if let Some(ref op_def) = *maybe_op_def {
            encode_operand(&mut buffer, &op_def, op, mode, addr_size)?;
        }
    }

    buffer.write(writer, mode)
}

fn get_operand_size_prefix(behavior: OperandSizePrefixBehavior, mode: Mode) 
    -> bool {
    match behavior {
        OperandSizePrefixBehavior::Always => true,
        OperandSizePrefixBehavior::Never => false,
        OperandSizePrefixBehavior::RealOnly => mode == Mode::Real,
        OperandSizePrefixBehavior::NotReal => mode != Mode::Real,
    }
}

fn get_addr_size(def: &InstructionDefinition, instr: &Instruction, mode: Mode)
    -> Result<OperandSize, InstructionEncodingError> {
    match check_unique::<_, OperandSize, InstructionEncodingError>(def.operands.iter().zip(instr.operands().iter()).filter_map(
        |(ref op_def, op)| op_def.as_ref().and_then( // TODO Clean up?
            |_| get_operand_addr_size(op)
                .map(|v| v.map(Ok)).unwrap_or_else(|e| Some(Err(e))))
            ), InstructionEncodingError::InvalidAddressing) {
        Ok(Some(size)) => Ok(size),
        Ok(None) => Ok(mode.pointer_size()),
        Err(_) => Err(InstructionEncodingError::InvalidAddressing)
    }
}

fn get_operand_addr_size(operand: &Option<Operand>)
    -> Result<Option<OperandSize>, InstructionEncodingError> {
    operand.map(|op| match op {
        // TODO - Should offset/memory be here?
        Operand::Indirect(reg, ..) |
        Operand::IndirectScaledDisplaced(reg, ..) |
        Operand::IndirectDisplaced(reg, ..) => Ok(Some(reg.size())),
        Operand::IndirectScaledIndexed(base, index, ..) |
        Operand::IndirectScaledIndexedDisplaced(base, index, ..) => { 
            let base_size = base.size();
            let index_size = index.size();
            if base_size == index_size { Ok(Some(base_size)) }
            else { Err(InstructionEncodingError::InvalidAddressing) }
        },
        _ => Ok(None)
    }).unwrap_or(Ok(None))
}

// 3 Cases:
// - Unambiguous - Ok(Some(val))
// - Ambiguous - Ok(None)
// - Conflicting - Err(_)
fn check_unique<I, J, E>(mut iter: I, default_error: E) -> Result<Option<J>, E>
    where J: PartialEq, I: Iterator<Item=Result<J, E>> {
    if let Some(first_res) = iter.next() {
        first_res.and_then(|first| {
            for res in iter {
                match res {
                    Ok(item) => if item != first { return Err(default_error); },
                    Err(e) => { return Err(e); }
                }
            }
            Ok(Some(first))
        })
    } else { Ok(None) }
}

fn get_address_size_prefix(addr_size: OperandSize, mode: Mode) -> bool {
    match mode {
        Mode::Long => match addr_size {
            OperandSize::Qword => false,
            OperandSize::Dword => true,
            _ => unreachable!()
        },
        Mode::Protected => match addr_size {
            OperandSize::Dword => false,
            OperandSize::Word => true,
            _ => unreachable!()
        },
        Mode::Real => match addr_size {
            OperandSize::Word => false,
            OperandSize::Dword => true,
            _ => unreachable!()
        },
    }
}

pub fn get_op_size_64(def: &InstructionDefinition) -> bool {
    if let Some(CompositePrefix::Rex { size_64: Some(true) }) = def.composite_prefix
        { true } else { false }
}

pub fn encode_operand(buffer: &mut InstructionBuffer, def: &OperandDefinition, op: &Option<Operand>,
    mode: Mode, addr_size: OperandSize) -> Result<(), InstructionEncodingError> {
    if let OperandType::Fixed(_) = def.op_type { return Ok(()); }

    match def.encoding {
        OperandEncoding::ModRmReg => { 
            if let Some(Operand::Direct(reg)) = *op {
                buffer.mod_rm_reg = Some(reg.get_reg_code());

                if reg.needs_rex() && buffer.composite_prefix.is_none() {
                    buffer.composite_prefix = Some(::instruction_buffer::CompositePrefix::Rex);
                }
            } else { panic!("Internal error."); }
        }
        OperandEncoding::Mib |
        OperandEncoding::ModRmRm => { encode_rm(buffer, &op.expect("Internal error."), mode)?; },
        OperandEncoding::Vex => {
            if let Some(Operand::Direct(reg)) = *op {
                buffer.vex_operand = Some(reg.get_reg_code());
            } else { panic!("Internal error."); }
        },
        OperandEncoding::Offset |
        OperandEncoding::Imm => {
            match op.expect("Internal error.") {
                Operand::Literal8(val) => 
                    { buffer.add_immediate(ImmediateValue::Literal8(val)); },
                Operand::Literal16(val) =>
                    { buffer.add_immediate(ImmediateValue::Literal16(val)); },
                Operand::Literal32(val) =>
                    { buffer.add_immediate(ImmediateValue::Literal32(val)); },
                Operand::Literal64(val) =>
                    { buffer.add_immediate(ImmediateValue::Literal64(val)); },
                Operand::MemoryAndSegment16(seg, addr) =>
                    { buffer.add_immediate(ImmediateValue::MemoryAndSegment16(seg, addr)); },
                Operand::MemoryAndSegment32(seg, addr) =>
                    { buffer.add_immediate(ImmediateValue::MemoryAndSegment32(seg, addr)); },
                Operand::Offset(offset, ..) => { 
                    let imm_size = if let OperandType::Rel(enc_size) = def.op_type { enc_size }
                        else { addr_size };
                    buffer.add_immediate(match imm_size {
                        OperandSize::Byte => { ImmediateValue::Literal8(offset as u8) },
                        OperandSize::Word => { ImmediateValue::Literal16(offset as u16) },
                        OperandSize::Dword => { ImmediateValue::Literal32(offset as u32) },
                        OperandSize::Qword => { ImmediateValue::Literal64(offset) },
                        _ => panic!("Internal error")
                    });
                },
                Operand::Direct(reg) => 
                    { buffer.add_immediate(ImmediateValue::Literal8(reg.get_reg_code() << 4)); }
                _ => panic!("Invalid immediate operand: {:?}.", op)
            }
        },
        OperandEncoding::OpcodeAddend => {
            if let Some(Operand::Direct(reg)) = *op {
                buffer.opcode_add = Some(reg.get_reg_code());
            } else { panic!("Internal error."); }
        },
        OperandEncoding::Fixed => {}
    }

    Ok(())
}

fn encode_rm(buffer: &mut InstructionBuffer, op: &Operand, mode: Mode)
    -> Result<(), InstructionEncodingError> {
    match *op {
        Operand::Direct(reg) => {
            buffer.mod_rm_mod = Some(0b11);
            buffer.mod_rm_rm = Some(reg.get_reg_code());
            Ok(())
        },
        Operand::Indirect(base, ..) => {
            encode_indirect(buffer, Some(base), None, None, 0, mode)
        },
        Operand::IndirectDisplaced(base, disp, ..) => {
            encode_indirect(buffer, Some(base), None, None, disp, mode)
        },
        Operand::IndirectScaledIndexed(base, index, scale, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), 0, mode)
        },
        Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode)
        },
        Operand::IndirectScaledDisplaced(index, scale, disp, ..) => {
            encode_indirect(buffer, None, Some(index), Some(scale), disp, mode)
        },
        Operand::Memory(addr, ..) | Operand::Offset(addr, ..) => { // TODO Should offset panic or be here?
            encode_indirect(buffer, None, None, None, addr, mode)
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
            encode_indirect_16(buffer, base, index, displacement).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_32(buffer, base, index, scale, displacement, mode)
            })
        },
        Mode::Protected => {
            encode_indirect_32(buffer, base, index, scale, displacement, mode).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_16(buffer, base, index, displacement)
            })
        },
        Mode::Long => {
            encode_indirect_64(buffer, base, index, scale, displacement).or_else(|_| {
                buffer.address_size_prefix = true;
                encode_indirect_32(buffer, base, index, scale, displacement, mode)
            })
        }
    }
}

fn encode_indirect_16(buffer: &mut InstructionBuffer, reg1: Option<Reg>, reg2: Option<Reg>, displacement: u64) -> Result<(), InstructionEncodingError> {
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
    } else if displacement < 128 as u64 {
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
        else if disp <= 128 as u64 {
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
                    if displacement < 128 as u64 {
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
                    if displacement < 128 as u64 {
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

fn encode_indirect_64(buffer: &mut InstructionBuffer, base: Option<Reg>, index: Option<Reg>, scale: Option<RegScale>, displacement: u64) -> Result<(), InstructionEncodingError> {
    fn disp_helper(buffer: &mut InstructionBuffer, disp: u64) {
        if disp == 0 { buffer.mod_rm_mod = Some(0); }
        else if disp <= 128 as u64 {
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
                    if displacement < 128 as u64 {
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
                    if displacement < 128 as u64 {
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
