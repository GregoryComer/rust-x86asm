use std::io::Write;
use ::{Instruction, Mnemonic, Mode, Operand, OperandSize, Reg, RegScale};
use ::instruction_buffer::{ImmediateValue, InstructionBuffer, Prefix1, Prefix2};
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
    unimplemented!();
}

pub fn encode_operand(buffer: &mut InstructionBuffer, def: &OperandDefinition, op: &Option<Operand>, mode: Mode) -> Result<(), InstructionEncodingError> {
    unimplemented!();
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
        Operand::IndirectDisplaced(base, disp, ..) => {
            encode_indirect(buffer, Some(base), None, None, disp, mode);
        },
        Operand::IndirectScaledIndexed(base, index, scale, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), 0, mode);
        },
        Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, ..) => {
            encode_indirect(buffer, Some(base), Some(index), Some(scale), disp, mode);
        },
        Operand::IndirectScaledDisplaced(index, scale, disp, ..) => {
            encode_indirect(buffer, None, Some(index), Some(scale), disp, mode);
        },
        Operand::Memory(addr, ..) | Operand::Offset(addr, ..) => { // TODO Should offset panic or be here?
            encode_indirect(buffer, None, None, None, addr, mode);
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
        _ => panic!("Invalid operand.")
    }
}

fn encode_vvvv(buffer: &mut InstructionBuffer, op: &Operand) {
    match *op {
        Operand::Direct(reg) => { buffer.vex_operand = Some(reg.get_reg_code()); }
        _ => panic!("Invalid operand.")
    }
}
