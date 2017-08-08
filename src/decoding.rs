use std::io::{Bytes, Read};
use std::iter::Peekable;
use ::{Instruction, InstructionFlags, MergeMode, Mnemonic, Mode, Operand, OperandSize, ProcessorLevel, Reg, RegScale, SegmentReg};
use ::instruction_buffer::*;
use ::instruction_def::*;
use arrayvec::ArrayVec;

pub struct InstructionReader<T: Read> {
    reader: Peekable<Bytes<T>>,
    mode: Mode,
    proc_level: ProcessorLevel
}

impl<T: Read> InstructionReader<T> {
    pub fn new(reader: T, mode: Mode) -> InstructionReader<T> {
        InstructionReader {
            reader: reader.bytes().peekable(),
            mode: mode,
            proc_level: ProcessorLevel::Corei7
        }
    }

    fn expect_byte(&mut self) -> Result<u8, InstructionDecodingError> {
        match self.reader.next() {
            Some(Ok(b)) => Ok(b),
            Some(Err(_)) => Err(InstructionDecodingError::ReadError),
            None => Err(InstructionDecodingError::PartialInstruction)
        }
    }
    
    fn expect_byte_peek(&mut self) -> Result<u8, InstructionDecodingError> {
        match self.reader.peek() {
            Some(&Ok(b)) => Ok(b),
            Some(&Err(_)) => Err(InstructionDecodingError::ReadError),
            None => Err(InstructionDecodingError::PartialInstruction)
        }
    }

    pub fn read(&mut self) -> Result<Instruction, InstructionDecodingError> {
        let mut buffer: InstructionBuffer = Default::default();
        let mut reg_ext = 0; // Extension to mod_rm_reg field
        let mut index_ext = 0; // Extension to sib_index field
        let mut b_ext = 0; // Extension to mod_rm_rm field or sib_base field
        let mut v_ext = 0; // Extension to vex operand field
        let mut opcode_byte = 0; // Primary opcode

        // Check for end of stream
        if self.reader.peek().is_none() {
            return Err(InstructionDecodingError::EndOfStream);
        }

        // Read prefixes
        loop {
            match self.expect_byte()? {
                PREFIX_LOCK => { buffer.prefix1 = Some(Prefix1::Lock); },
                PREFIX_REPNE => { buffer.prefix1 = Some(Prefix1::RepNE); },
                PREFIX_REP => { buffer.prefix1 = Some(Prefix1::Rep); },
                // TODO Remaining rep prefixes?
                PREFIX_OP_SIZE => { buffer.operand_size_prefix = true; },
                PREFIX_ADDR_SIZE => { buffer.address_size_prefix = true; },
                PREFIX_CS => { buffer.prefix2 = Some(Prefix2::CS); },
                PREFIX_SS => { buffer.prefix2 = Some(Prefix2::SS); },
                PREFIX_DS => { buffer.prefix2 = Some(Prefix2::DS); },
                PREFIX_ES => { buffer.prefix2 = Some(Prefix2::ES); },
                PREFIX_FS => { buffer.prefix2 = Some(Prefix2::FS); },
                PREFIX_GS => { buffer.prefix2 = Some(Prefix2::GS); },
                PREFIX_BRANCH_NOT_TAKEN => { buffer.prefix2 = Some(Prefix2::BranchNotTaken); },
                PREFIX_BRANCH_TAKEN => { buffer.prefix2 = Some(Prefix2::BranchTaken); },
                PREFIX_TWO_BYTE_OPCODE => { buffer.is_two_byte_opcode = true; },
                PREFIX_VEX2 => { // Two-byte VEX prefix
                    let data = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::VEX);
                    reg_ext = if data & 0x80 != 0 { 0 } else { 0x8 };
                    buffer.vex_operand = Some((data >> 3) & 0xF);
                    buffer.vex_l = Some(data & 0x2 != 0);
                    match data & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.fixed_prefix = Some(0xF3); },
                        0x3 => { buffer.fixed_prefix = Some(0xF2); },
                        _ => {}
                    }
                },
                PREFIX_VEX3 => { // Three-byte VEX prefix
                    let data1 = self.expect_byte()?;
                    let data2 = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::VEX);
                    reg_ext = if data1 & 0x80 != 0 { 0 } else { 0x8 };
                    index_ext = if data1 & 0x40 != 0 { 0 } else { 0x8 };
                    b_ext = if data1 & 0x20 != 0 { 0 } else { 0x8 };
                    match data1 & 0x1F { // map_select
                        0 => {},
                        1 => buffer.is_two_byte_opcode = true,
                        2 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x38; },
                        3 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x3A; },
                        _ => return Err(InstructionDecodingError::InvalidInstruction)
                    }
                    buffer.vex_e = Some(data2 & 0x80 != 0);
                    buffer.vex_operand = Some((data2 >> 3) & 0xF);
                    buffer.vex_l = Some(data2 & 0x2 != 0);
                    match data2 & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.fixed_prefix = Some(0xF3); },
                        0x3 => { buffer.fixed_prefix = Some(0xF2); },
                        _ => {}
                    }
                },
                PREFIX_EVEX => {
                    let data1 = self.expect_byte()?;
                    let data2 = self.expect_byte()?;
                    let data3 = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::EVEX);
                    reg_ext |= if data1 & 0x80 != 0 { 0x8 } else { 0 };
                    index_ext |= if data1 & 0x40 != 0 { 0x8 } else { 0 };
                    b_ext |= if data1 & 0x20 != 0 { 0x8 } else { 0 };
                    reg_ext |= if data1 & 0x10 != 0 { 0x10 } else { 0 };
                    match data1 & 0x3 { // map_select
                        0 => {},
                        1 => buffer.is_two_byte_opcode = true,
                        2 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x38; },
                        3 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x3A; },
                        _ => return Err(InstructionDecodingError::InvalidInstruction)
                    }
                    buffer.vex_e = Some(data2 & 0x80 != 0);
                    buffer.vex_operand = Some((data2 >> 3) & 0xF);
                    match data2 & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.fixed_prefix = Some(0xF3); },
                        0x3 => { buffer.fixed_prefix = Some(0xF2); },
                        _ => {}
                    }
                    buffer.merge_mode = Some(if data3 & 0x80 != 0 { MergeMode::Zero } else { MergeMode::Merge });
                    buffer.vex_l = Some(data3 & 0x40 != 0);
                    buffer.operand_size_64 = data3 & 0x20 != 0;
                    buffer.vex_b = Some(data3 & 0x10 != 0);
                    v_ext = if data3 & 0x8 != 0 { 0x10 } else { 0x0 };
                    buffer.mask_reg = Some(data3 & 0x7);
                },
                b if self.mode == Mode::Long && b & 0xF0 == 0x40 => { // REX prefix
                    buffer.composite_prefix = Some(CompositePrefix::REX);
                    reg_ext |= if b & 0x4 != 0 { 0x8 } else { 0 };
                    index_ext |= if b & 0x2 != 0 { 0x8 } else { 0 };
                    b_ext |= if b & 0x1 != 0 { 0x8 } else { 0 };
                    buffer.operand_size_64 = b & 0x8 != 0;
                },
                b => { opcode_byte = b; break; }, // Not a prefix, move on
            }
        }

        let addr_mode = Mode::from_size(InstructionReader::<T>::get_addressing_mode(self.mode, &buffer)).unwrap();

        // Read opcode
        if buffer.primary_opcode == 0 {
            buffer.primary_opcode = opcode_byte;
        } else {
            buffer.secondary_opcode = Some(opcode_byte);
        }

        if (buffer.primary_opcode == 0x38 || buffer.primary_opcode == 0x3A) && buffer.secondary_opcode.is_none() {
            buffer.secondary_opcode = Some(self.expect_byte()?);
        }

        // Find the matching instruction definition
        let def = get_instruction_def_by_opcode(buffer.primary_opcode, buffer.secondary_opcode,
            buffer.is_two_byte_opcode).ok_or(InstructionDecodingError::UnknownOpcode)?;

        // ModR/M
        if InstructionReader::<T>::has_mod_rm(def) {
            let mod_rm = self.expect_byte()?;
            buffer.mod_rm_mod = Some(mod_rm >> 6);
            buffer.mod_rm_reg = Some((mod_rm >> 3) & 0x7 | reg_ext);
            buffer.mod_rm_rm = Some(mod_rm & 0x7);

            // SIB
            if InstructionReader::<T>::has_sib(addr_mode, &buffer) {
                let sib = self.expect_byte()?;
                buffer.sib_scale = Some(sib >> 6);
                buffer.sib_index = Some((mod_rm >> 3) & 0x7 | index_ext);
                buffer.sib_base = Some(mod_rm & 0x7 | b_ext);
            } else {
                buffer.mod_rm_reg = buffer.mod_rm_reg.map(|reg| reg | b_ext);
                buffer.mod_rm_rm = buffer.mod_rm_rm.map(|rm| rm | index_ext);
            }

            // Displacement
            if let Some(disp_size) = InstructionReader::<T>::get_displacement_size(addr_mode, &buffer) {
                match disp_size {
                    OperandSize::Byte => {
                        buffer.displacement = Some(ImmediateValue::Literal8(self.expect_byte()?));
                    },
                    OperandSize::Word => {
                        buffer.displacement = Some(ImmediateValue::Literal16(
                            (self.expect_byte()? as u16) |
                            (self.expect_byte()? as u16) << 8));
                    },
                    OperandSize::Dword => {
                        buffer.displacement = Some(ImmediateValue::Literal32(
                            (self.expect_byte()? as u32) |
                            (self.expect_byte()? as u32) << 8 |
                            (self.expect_byte()? as u32) << 16 |
                            (self.expect_byte()? as u32) << 24));
                    },
                    _ => panic!("Invalid displacement size.") // Shouldn't ever happen
                }
            }
        }

        println!("Buffer: {:?}", buffer);

        // Build operands (read immediates as appropriate)
        // Destination operand is typically operand1, but needs to be read last (source operands
        // come first), so read the operands in the correct order (ordered_operands()), then
        // re-arrange them so they map to the correct operands.
        let operand_results: Result<ArrayVec<[_; 4]>, InstructionDecodingError> = def.ordered_operands().iter()
            .map(|op_def| op_def.as_ref().map_or(Ok(None), |o_d| Ok(Some(self.read_operand(o_d, &buffer)?)))).collect();
        let operands = operand_results?;
        let ordered_operands = if def.has_destination {
            [operands[3], operands[0], operands[1], operands[2]]
        } else {
            [operands[0], operands[1], operands[2], operands[3]]
        };

        // Build instruction
        let mut instr = Instruction {
            mnemonic: def.mnemonic,
            operand1: ordered_operands[0],
            operand2: ordered_operands[1],
            operand3: ordered_operands[2],
            operand4: ordered_operands[3],
            flags: InstructionFlags {
                lock: buffer.prefix1.map(|p| p == Prefix1::Lock).unwrap_or(false),
                rounding_mode: None, // TODO
                sae: false, // TODO
            }
        };

        Ok(instr)
    }

    fn read_literal8(&mut self) -> Result<Operand, InstructionDecodingError> {
        self.expect_byte().map(|b| Operand::Literal8(b))
    }

    fn read_literal16(&mut self) -> Result<Operand, InstructionDecodingError> {
        (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u16) << (8*n) )))).map(|b| Operand::Literal16(b))
    }

    fn read_literal32(&mut self) -> Result<Operand, InstructionDecodingError> {
        (0..4).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u32) << (8*n) )))).map(|b| Operand::Literal32(b))
    }

    fn read_literal64(&mut self) -> Result<Operand, InstructionDecodingError> {
        (0..8).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u64) << (8*n) )))).map(|b| Operand::Literal64(b))
    }

    fn read_literal_sized(&mut self, size: OperandSize) -> Result<Operand, InstructionDecodingError> {
        match size {
            OperandSize::Byte => self.read_literal8(),
            OperandSize::Word => self.read_literal16(),
            OperandSize::Dword => self.read_literal32(),
            OperandSize::Qword => self.read_literal64(),
            _ => panic!("Invalid literal size.")
        }
    }

    fn read_memory_and_segment_16(&mut self) -> Result<Operand, InstructionDecodingError> {
        let addr = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u16) << (8*n) ))));
        let segment = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u16) << (8*n) ))));
        segment.and_then(|s| addr.map(|a| Operand::MemoryAndSegment16(s, a)))
    }

    fn read_memory_and_segment_32(&mut self) -> Result<Operand, InstructionDecodingError> {
        let addr = (0..4).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u32) << (8*n) ))));
        let segment = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u16) << (8*n) ))));
        segment.and_then(|s| addr.map(|a| Operand::MemoryAndSegment32(s, a)))
    }

    fn read_disp8(&mut self) -> Result<u8, InstructionDecodingError> {
        self.expect_byte()
    }

    fn read_disp16(&mut self) -> Result<u16, InstructionDecodingError> {
        (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u16) << (8*n) ))))
    }

    fn read_disp32(&mut self) -> Result<u32, InstructionDecodingError> {
        (0..4).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u32) << (8*n) ))))
    }

    fn read_disp64(&mut self) -> Result<u64, InstructionDecodingError> {
        (0..8).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u64) << (8*n) ))))
    }

    fn read_disp_sized(&mut self, size: OperandSize) -> Result<u64, InstructionDecodingError> {
        match size {
            OperandSize::Byte => self.read_disp8().map(|v| v as u64),
            OperandSize::Word => self.read_disp16().map(|v| v as u64),
            OperandSize::Dword => self.read_disp32().map(|v| v as u64),
            OperandSize::Qword => self.read_disp64(),
            _ => panic!("Invalid displacement size.")
        }
    }

    fn read_operand(&mut self, op_def: &OperandDescription, buffer: &InstructionBuffer)
        -> Result<Operand, InstructionDecodingError> {
        let size = InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer);
        let addr_size = InstructionReader::<T>::get_address_size(self.mode, buffer);
        Ok(match op_def.addressing_mode {
            OperandAddressingMode::A => {
                if (!buffer.address_size_prefix && self.mode == Mode::Real)
                    || (buffer.address_size_prefix && self.mode != Mode::Real) {
                        self.read_memory_and_segment_16()?
                    } else { // 32-bit address
                        self.read_memory_and_segment_32()?
                    }
            },
            OperandAddressingMode::BA => Operand::Indirect(if self.mode == Mode::Long { Reg::RAX }
                else { Reg::EAX }, Some(InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer)), Some(SegmentReg::DS)),
            OperandAddressingMode::BB => Operand::IndirectScaledIndexed(InstructionReader::<T>::get_sized_b(self.mode, &buffer, addr_size), Reg::AL, RegScale::One, Some(InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer)), Some(SegmentReg::DS)),
            OperandAddressingMode::BD => Operand::Indirect(InstructionReader::<T>::get_sized_di(self.mode, &buffer, InstructionReader::<T>::get_address_size(self.mode, &buffer)), Some(size), Some(SegmentReg::DS)),
            OperandAddressingMode::C => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_control)?),
            OperandAddressingMode::D => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_debug)?),
            OperandAddressingMode::E |
            OperandAddressingMode::M => self.rm_helper(buffer, op_def, |code| Reg::from_code_general_sized(code, InstructionReader::<T>::has_rex(buffer), size))?,
            OperandAddressingMode::ES => self.rm_helper(buffer, op_def, Reg::from_code_fpu)?, 
            OperandAddressingMode::EST => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_rm_code, Reg::from_code_fpu)?),
            OperandAddressingMode::F => Operand::Direct(InstructionReader::<T>::get_flags_sized(size)),
            OperandAddressingMode::G => Operand::Direct(InstructionReader::<T>::reg_helper_general(self.mode, buffer, op_def, InstructionReader::<T>::get_reg_code)?),
            OperandAddressingMode::H => Operand::Direct(Reg::from_code_general_sized(InstructionReader::<T>::get_rm_code(buffer)?, InstructionReader::<T>::has_rex(buffer), size).ok_or(InstructionDecodingError::InvalidInstruction)?),
            OperandAddressingMode::I |
            OperandAddressingMode::J => self.read_literal_sized(size)?,
            OperandAddressingMode::N => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_rm_code, Reg::from_code_mmx)?),
            OperandAddressingMode::O => Operand::Offset(self.read_disp_sized(size)?, Some(size), buffer.get_segment_reg()),
            OperandAddressingMode::P => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_mmx)?),
            OperandAddressingMode::Q => self.rm_helper(buffer, op_def, Reg::from_code_mmx)?,
            OperandAddressingMode::R => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_mod_code, |code| Reg::from_code_general_sized(code, InstructionReader::<T>::has_rex(buffer), size))?),
            OperandAddressingMode::S => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_segment)?),
            OperandAddressingMode::SC => unimplemented!(), // TODO?
            OperandAddressingMode::T => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_test)?),
            OperandAddressingMode::U => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_rm_code, Reg::from_code_xmm)?),
            OperandAddressingMode::V => Operand::Direct(InstructionReader::<T>::reg_helper(buffer, InstructionReader::<T>::get_reg_code, Reg::from_code_xmm)?),
            OperandAddressingMode::W => self.rm_helper(buffer, op_def, Reg::from_code_xmm)?,
            OperandAddressingMode::X => match addr_size {
                    OperandSize::Word => Operand::Indirect(Reg::SI, Some(size), Some(SegmentReg::DS)),
                    OperandSize::Dword => Operand::Indirect(Reg::ESI, Some(size), Some(SegmentReg::DS)),
                    OperandSize::Qword => Operand::Indirect(Reg::RSI, Some(size), Some(SegmentReg::DS)),
                    _ => unreachable!()
                },
            OperandAddressingMode::Y => match addr_size {
                    OperandSize::Word => Operand::Indirect(Reg::DI, Some(size), Some(SegmentReg::ES)),
                    OperandSize::Dword => Operand::Indirect(Reg::EDI, Some(size), Some(SegmentReg::ES)),
                    OperandSize::Qword => Operand::Indirect(Reg::RDI, Some(size), Some(SegmentReg::ES)),
                    _ => unreachable!()
                },
            OperandAddressingMode::Z => Operand::Direct(Reg::from_code_general_sized(buffer.primary_opcode & 0x7,
                InstructionReader::<T>::has_rex(buffer), size).ok_or(InstructionDecodingError::InvalidInstruction)?),
            OperandAddressingMode::AVXVex |
            OperandAddressingMode::AVXMemRm |
            OperandAddressingMode::AVXReg |
            OperandAddressingMode::AVXRm |
            OperandAddressingMode::AVXMaskedReg |
            OperandAddressingMode::AVXMemBcst32Rm |
            OperandAddressingMode::AVXMemBcst64Rm |
            OperandAddressingMode::AVXImm8 |
            OperandAddressingMode::AVXDestMemRm |
            OperandAddressingMode::AVXRegMaskedRm |
            OperandAddressingMode::MaskedMaskReg |
            OperandAddressingMode::MaskReg |
            OperandAddressingMode::MaskRm |
            OperandAddressingMode::MaskMemRm |
            OperandAddressingMode::MaskVex |
            OperandAddressingMode::GeneralVex |
            OperandAddressingMode::GeneralRm |
            OperandAddressingMode::BoundReg |
            OperandAddressingMode::BoundMemRm |
            OperandAddressingMode::Fixed => unimplemented!(),
        })
    }

    fn has_mod_rm(def: &InstructionDefinition) -> bool {
        def.ordered_operands().iter().filter_map(|x| **x).any(|op_def| match op_def.addressing_mode {
            OperandAddressingMode::A |
            OperandAddressingMode::BA |
            OperandAddressingMode::BB |
            OperandAddressingMode::BD |
            OperandAddressingMode::F |
            OperandAddressingMode::I |
            OperandAddressingMode::J |
            OperandAddressingMode::O |
            OperandAddressingMode::W |
            OperandAddressingMode::X |
            OperandAddressingMode::Y |
            OperandAddressingMode::Z |
            OperandAddressingMode::Fixed
                => false,

            OperandAddressingMode::C |
            OperandAddressingMode::D |
            OperandAddressingMode::E |
            OperandAddressingMode::ES |
            OperandAddressingMode::EST |
            OperandAddressingMode::G |
            OperandAddressingMode::H |
            OperandAddressingMode::M |
            OperandAddressingMode::N |
            OperandAddressingMode::P |
            OperandAddressingMode::Q |
            OperandAddressingMode::R |
            OperandAddressingMode::S |
            OperandAddressingMode::SC |
            OperandAddressingMode::T |
            OperandAddressingMode::U |
            OperandAddressingMode::V |
            OperandAddressingMode::W |
            OperandAddressingMode::AVXVex |
            OperandAddressingMode::AVXMemRm |
            OperandAddressingMode::AVXReg |
            OperandAddressingMode::AVXRm |
            OperandAddressingMode::AVXMaskedReg |
            OperandAddressingMode::AVXMemBcst32Rm |
            OperandAddressingMode::AVXMemBcst64Rm |
            OperandAddressingMode::AVXImm8 |
            OperandAddressingMode::AVXDestMemRm |
            OperandAddressingMode::AVXRegMaskedRm |
            OperandAddressingMode::MaskedMaskReg |
            OperandAddressingMode::MaskReg |
            OperandAddressingMode::MaskRm |
            OperandAddressingMode::MaskMemRm |
            OperandAddressingMode::MaskVex |
            OperandAddressingMode::GeneralVex |
            OperandAddressingMode::GeneralRm |
            OperandAddressingMode::BoundReg |
            OperandAddressingMode::BoundMemRm 
                => true
        })
    }

    fn get_address_size(mode: Mode, buffer: &InstructionBuffer) -> OperandSize {
        match mode {
            Mode::Real => if !buffer.address_size_prefix { OperandSize::Word } else { OperandSize::Dword },
            Mode::Protected => if !buffer.address_size_prefix { OperandSize::Dword } else { OperandSize::Word },
            Mode::Long => if !buffer.address_size_prefix { OperandSize::Qword } else { OperandSize::Dword },
        }
    }

    fn get_reg_code(buffer: &InstructionBuffer) -> Result<u8, InstructionDecodingError> {
        buffer.mod_rm_reg.ok_or(InstructionDecodingError::PartialInstruction)
    }

    fn get_rm_code(buffer: &InstructionBuffer) -> Result<u8, InstructionDecodingError> {
        buffer.mod_rm_rm.ok_or(InstructionDecodingError::PartialInstruction)
    }

    fn get_mod_code(buffer: &InstructionBuffer) -> Result<u8, InstructionDecodingError> {
        buffer.mod_rm_mod.ok_or(InstructionDecodingError::PartialInstruction)
    }

    fn get_vex_code(buffer: &InstructionBuffer) -> Result<u8, InstructionDecodingError> {
        buffer.mod_rm_rm.ok_or(InstructionDecodingError::PartialInstruction)
    }

    fn get_sized_a(mode: Mode, buffer: &InstructionBuffer, size: OperandSize) -> Reg {
        match size {
            OperandSize::Word => Reg::AX,
            OperandSize::Dword => Reg::EAX,
            OperandSize::Qword => Reg::RAX,
            _ => panic!("Invalid address size.")
        }
    }

    fn get_sized_b(mode: Mode, buffer: &InstructionBuffer, size: OperandSize) -> Reg {
        match size {
            OperandSize::Word => Reg::BX,
            OperandSize::Dword => Reg::EBX,
            OperandSize::Qword => Reg::RBX,
            _ => panic!("Invalid address size.")
        }
    }

    fn get_sized_di(mode: Mode, buffer: &InstructionBuffer, size: OperandSize) -> Reg {
        match size {
            OperandSize::Word => Reg::DI,
            OperandSize::Dword => Reg::EDI,
            OperandSize::Qword => Reg::RDI,
            _ => panic!("Invalid address size.")
        }
    }

    fn get_flags_sized(size: OperandSize) -> Reg {
        match size {
            OperandSize::Word => Reg::FLAGS,
            OperandSize::Dword => Reg::EFLAGS,
            OperandSize::Qword => Reg::RFLAGS,
            _ => panic!("Invalid flags register size.\r\n")
        }
    }

    fn has_sib(mode: Mode, buffer: &InstructionBuffer) -> bool {
        (mode != Mode::Real) && (buffer.mod_rm_mod.and_then(|rm_mod| buffer.mod_rm_rm.map(
            |rm_rm| (rm_rm & 0b111) == 0b101)).unwrap_or(false))
    }

    fn get_displacement_size(mode: Mode, buffer: &InstructionBuffer) -> Option<OperandSize> {
        buffer.mod_rm_mod.and_then(|rm_mod| buffer.mod_rm_rm.and_then(|rm_rm|
            match rm_mod {
                0b00 => {
                    if rm_rm & 0b111 == 0b101 && mode != Mode::Real &&
                        (InstructionReader::<T>::has_sib(mode, buffer) || mode == Mode::Long) {
                            Some(OperandSize::Dword)
                    } else { None }
                },
                0b01 => Some(OperandSize::Byte),
                0b10 => Some(if mode == Mode::Real { OperandSize::Word } else { OperandSize::Dword }),
                _ => None
            }))
    }

    fn reg_helper<TConv>(buffer: &InstructionBuffer, read_proc: fn(&InstructionBuffer) -> 
                Result<u8, InstructionDecodingError>, conv_proc: TConv) -> Result<Reg, InstructionDecodingError>
                where TConv : Fn(u8) -> Option<Reg> {
        read_proc(buffer).and_then(|code| conv_proc(code).ok_or(InstructionDecodingError::InvalidInstruction))
    }

    fn reg_helper_general(mode: Mode, buffer: &InstructionBuffer, op_def: &OperandDescription, read_proc:
        fn(&InstructionBuffer) -> Result<u8, InstructionDecodingError>) -> Result<Reg, InstructionDecodingError> {
        match InstructionReader::<T>::get_operand_size(mode, op_def, buffer) {
            OperandSize::Byte => InstructionReader::<T>::reg_helper(buffer, read_proc,
                |code| Reg::from_code_general_8(code, InstructionReader::<T>::has_rex(buffer))),
            OperandSize::Word => InstructionReader::<T>::reg_helper(buffer, read_proc, Reg::from_code_general_16),
            OperandSize::Dword => InstructionReader::<T>::reg_helper(buffer, read_proc, Reg::from_code_general_32),
            OperandSize::Qword => InstructionReader::<T>::reg_helper(buffer, read_proc, Reg::from_code_general_64),
            _ => Err(InstructionDecodingError::InvalidInstruction)
        }
    }

    fn rm_helper<TConv>(&mut self, buffer: &InstructionBuffer, op_def: &OperandDescription, conv_proc: TConv)
        -> Result<Operand, InstructionDecodingError>
        where TConv : Fn(u8) -> Option<Reg> {
        let rm = buffer.mod_rm_rm.ok_or(InstructionDecodingError::InvalidInstruction)?;
        let addr_size = InstructionReader::<T>::get_address_size(self.mode, buffer);
        Ok(match addr_size {
            OperandSize::Word => {
                let mode = buffer.mod_rm_mod.ok_or(InstructionDecodingError::InvalidInstruction)?;
                let size = InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer);
                let segment = buffer.get_segment_reg();
                let disp = if mode == 0 && rm != 0b110 { None } // No displacement
                    else { // 8/16-bit displacement
                        Some((if mode == 1 { self.read_disp8()? as u64 } else { self.read_disp16()? as u64 }))
                    };
                let (reg1, reg2) = match rm {
                    0 => (Some(Reg::BX), Some(Reg::SI)),
                    1 => (Some(Reg::BX), Some(Reg::DI)),
                    2 => (Some(Reg::BP), Some(Reg::SI)),
                    3 => (Some(Reg::BP), Some(Reg::DI)),
                    4 => (Some(Reg::SI), None),
                    5 => (Some(Reg::DI), None),
                    6 => (None, None),
                    7 => (Some(Reg::BX), None),
                    _ => unreachable!()
                };
                match (reg1, reg2, disp) {
                    (None, None, Some(addr)) => Operand::Memory(addr, Some(size), segment),
                    (Some(r1), None, None) => Operand::Indirect(r1, Some(size), segment),
                    (Some(r1), None, Some(disp)) => Operand::IndirectDisplaced(r1, disp, Some(size), segment),
                    (Some(r1), Some(r2), None) =>
                        Operand::IndirectScaledIndexed(r1, r2, RegScale::One, Some(size), segment),
                    (Some(r1), Some(r2), Some(disp)) =>
                        Operand::IndirectScaledIndexedDisplaced(r1, r2, RegScale::One, disp, Some(size), segment),
                    _ => unreachable!()
                }
            },
            addr_size @ OperandSize::Dword | addr_size @ OperandSize::Qword => {
                let size = Some(InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer));
                let segment = buffer.get_segment_reg();
                match buffer.mod_rm_mod.ok_or(InstructionDecodingError::InvalidInstruction)? & 0x7 {
                    0b00 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b011 | 0b110 | 0b111 => // [RM]
                                Operand::Indirect(Reg::from_code_general_sized(rm, InstructionReader::<T>::has_rex(buffer),
                                    addr_size).ok_or(InstructionDecodingError::InvalidInstruction)?,
                                    size, segment),
                            0b100 => self.sib_helper(buffer, op_def, addr_size)?, // [SIB]
                            0b101 => unimplemented!(), // TODO [RIP/EIP + disp32]
                            _ => unreachable!()
                        }
                    },
                    0b01 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b100 | 0b101 | 0b110 | 0b111 => // [RM + disp8]
                                Operand::IndirectDisplaced(Reg::from_code_general_sized(rm, InstructionReader::<T>::has_rex(buffer),
                                    addr_size).ok_or(InstructionDecodingError::InvalidInstruction)?,
                                    self.read_disp32()? as u64, size, segment),
                            0b100 => self.sib_helper(buffer, op_def, addr_size)?, // [SIB + disp8]
                            _ => unreachable!()
                        }
                    },
                    0b10 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b100 | 0b101 | 0b110 | 0b111 => // [RM + disp32]
                                Operand::IndirectDisplaced(Reg::from_code_general_sized(rm, InstructionReader::<T>::has_rex(buffer),
                                    addr_size).ok_or(InstructionDecodingError::InvalidInstruction)?,
                                    self.read_disp32()? as u64, size, segment),
                            0b100 => self.sib_helper(buffer, op_def, addr_size)?, // [SIB + disp32]
                            _ => unreachable!()
                        }
                    },
                    0b11 => Operand::Direct(conv_proc(rm).ok_or(InstructionDecodingError::InvalidInstruction)?),
                    _ => panic!("Invalid mod_rm_mod value.") // Should be statically impossible
                }
            },
            _ => unreachable!()
        })
    }

    fn sib_helper(&mut self, buffer: &InstructionBuffer, op_def: &OperandDescription, addr_size: OperandSize) -> 
        Result<Operand, InstructionDecodingError> {
        let scale = buffer.sib_scale.and_then(|scale| RegScale::from_sib_code(scale))
            .ok_or(InstructionDecodingError::InvalidInstruction)?;
        let index_code = buffer.sib_index.ok_or(InstructionDecodingError::InvalidInstruction)?;
        let index = Reg::from_code_general_sized(index_code, InstructionReader::<T>::has_rex(buffer), addr_size)
            .ok_or(InstructionDecodingError::InvalidInstruction)?;
        let base_code = buffer.sib_base.ok_or(InstructionDecodingError::InvalidInstruction)?;
        let base = Reg::from_code_general_sized(base_code, InstructionReader::<T>::has_rex(buffer), addr_size)
            .ok_or(InstructionDecodingError::InvalidInstruction)?;
        let mode = buffer.mod_rm_mod.ok_or(InstructionDecodingError::InvalidInstruction)?;
        let segment = buffer.get_segment_reg();
        let size = Some(InstructionReader::<T>::get_operand_size(self.mode, op_def, buffer));
        
        Ok(match mode {
            0b00 => {
                if index_code == 0b100 { // [disp32]
                    if base_code & 0b111 == 0b101 {
                        Operand::Memory(self.read_disp32()? as u64, size, segment)
                    } else { // [base]
                        Operand::Indirect(base, size, segment)
                    }
                } else {
                    if base_code & 0b111 == 0b101 { // [index*s + disp32]
                        Operand::IndirectScaledDisplaced(index, scale, self.read_disp32()? as u64, size, segment)
                    } else { // [base + index*s]
                        Operand::IndirectScaledIndexed(base, index, scale, size, segment)
                    }
                }
            },
            0b01 | 0b10 => {
                let disp = if mode == 0b10 { self.read_disp8()? as u64 } else { self.read_disp32()? as u64 };
                if index_code == 0b100 { // [base + disp8/32]
                    Operand::IndirectDisplaced(base, disp, size, segment)
                } else { // [base + index*s + disp8/32]
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, size, segment)
                }
            },
            _ => panic!("Internal error.") // Should be statically impossible
        })
    }

    fn get_addressing_mode(mode: Mode, buffer: &InstructionBuffer) -> OperandSize {
        let prefix = buffer.address_size_prefix;
        match mode {
            Mode::Real => if prefix { OperandSize::Dword } else { OperandSize::Word },
            Mode::Protected => if prefix { OperandSize::Word } else { OperandSize::Dword }, 
            Mode::Long => if prefix { OperandSize::Qword } else { OperandSize::Dword }, 
        }
    }

    fn get_operand_size(mode: Mode, op_def: &OperandDescription, buffer: &InstructionBuffer) -> OperandSize {
        fn is_mem(buffer: &InstructionBuffer) -> bool {
            buffer.mod_rm_mod.map(|m| m != 0b11).unwrap_or(false)
        }

        match op_def.operand_type {
            OperandType::A => if (mode == Mode::Real) ^ (buffer.operand_size_prefix) { OperandSize::Dword } else { OperandSize::Qword },
            OperandType::B | OperandType::BS | OperandType::BSQ | OperandType::BSS =>
                OperandSize::Byte,
            OperandType::BCD | OperandType::ER | OperandType::PT | OperandType::T =>
                OperandSize::Tbyte,
            OperandType::D | OperandType::DI | OperandType::DO | OperandType::DS | OperandType::SI | OperandType::SR =>
                OperandSize::Dword,
            OperandType::DR | OperandType::PI | OperandType::PSQ | OperandType::Q | OperandType::QI | OperandType::QP =>
                OperandSize::Qword,
            OperandType::DQP => if buffer.operand_size_64 { OperandSize::Qword } else { OperandSize::Dword },
            OperandType::E | OperandType::ST | OperandType::STX => 
                OperandSize::Unsized,
            OperandType::P => if (mode == Mode::Real) ^ (buffer.operand_size_prefix) { OperandSize::Dword } else { OperandSize::Fword },
            OperandType::DQ | OperandType::PD | OperandType::PS | OperandType::SD | OperandType::SS =>
                OperandSize::XMMword,
            OperandType::PTP => if buffer.operand_size_64 { OperandSize::Tbyte } else if (mode == Mode::Real) ^ (buffer.operand_size_prefix)
                { OperandSize::Dword } else { OperandSize::Fword },
            OperandType::S => if mode != Mode::Long { OperandSize::Tbyte } else { OperandSize::Fword },
            OperandType::V | OperandType::VDS | OperandType::VS =>
                if (mode == Mode::Real) ^ (buffer.operand_size_prefix) { OperandSize::Word } else { OperandSize::Dword },
            OperandType::VQ => if buffer.operand_size_prefix { OperandSize::Qword } else { OperandSize::Word },
            OperandType::VQP => 
                if buffer.operand_size_64 { OperandSize::Qword } else if (mode == Mode::Real) ^ (buffer.operand_size_prefix)
                { OperandSize::Word } else { OperandSize::Dword },
            OperandType::W | OperandType::WI | OperandType::WO => OperandSize::Word,
            OperandType::XMM => OperandSize::XMMword,
            OperandType::YMM => OperandSize::YMMword,
            OperandType::ZMM => OperandSize::ZMMword,
            OperandType::XMMorYMM => if buffer.vector_len.unwrap_or(false) { OperandSize::YMMword } else { OperandSize::XMMword },
            OperandType::XMMorYMMorMem32 => if is_mem(buffer) && buffer.vex_b.unwrap_or(false) { OperandSize::Dword } 
                else if buffer.vector_len.unwrap_or(false) { OperandSize::YMMword } else { OperandSize::XMMword },
            OperandType::XMMorYMMorMemOrMem64 => if is_mem(buffer) && buffer.vex_b.unwrap_or(true) { OperandSize::Qword }
                else { if buffer.vector_len.unwrap_or(false) { OperandSize::YMMword } else { OperandSize::XMMword } },
            OperandType::XMMorMem32 | OperandType::XMMorMemOrMem32 => 
                if is_mem(buffer) && buffer.vex_b.unwrap_or(false) { OperandSize::Dword }
                else { OperandSize::XMMword },
            OperandType::XMMorMem64 | OperandType::XMMorMemOrMem64 => if is_mem(buffer) && buffer.vex_b.unwrap_or(false) 
                { OperandSize::Qword } else { OperandSize::XMMword },
            OperandType::YMMorMemOrMem32 => if is_mem(buffer) && buffer.vex_b.unwrap_or(false) 
                { OperandSize::Dword } else { OperandSize::YMMword },
            OperandType::YMMorMemOrMem64 => if is_mem(buffer) && buffer.vex_b.unwrap_or(false) 
                { OperandSize::Qword } else { OperandSize::YMMword },
            OperandType::ZMMorMemOrMem64 => if is_mem(buffer) && buffer.vex_b.unwrap_or(false) 
                { OperandSize::Qword } else { OperandSize::ZMMword },
            OperandType::AVX => match (buffer.vector_len.unwrap_or(false), buffer.vex_l.unwrap_or(false)) {
                    (false, _) => OperandSize::XMMword,
                    (true, false) => OperandSize::YMMword,
                    (true, true) => OperandSize::ZMMword
                },
            OperandType::MaskReg => match (buffer.operand_size_prefix, buffer.operand_size_64) {
                    (true, false) => OperandSize::Byte,
                    (false, false) => OperandSize::Word,
                    (true, true) => OperandSize::Dword,
                    (false, true) => OperandSize::Qword,
                },
            OperandType::MaskOrMem8 => OperandSize::Byte,
            OperandType::MaskOrMem16 => OperandSize::Word,
            OperandType::MaskOrMem32 => OperandSize::Dword,
            OperandType::MaskOrMem64 => OperandSize::Qword,
            OperandType::Bound => OperandSize::XMMword,
            OperandType::BoundOrMem => OperandSize::XMMword, // TODO?
            OperandType::UnsizedMemory => OperandSize::Unsized,
            OperandType::FpuRegister => OperandSize::Tbyte
        }
    }

    pub fn has_rex(buffer: &InstructionBuffer) -> bool {
        buffer.composite_prefix.as_ref().map(|p| *p == CompositePrefix::VEX).unwrap_or(false)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstructionDecodingError {
    // EndOfStream - Indicates no more bytes are available on the underlying stream. Not returned when partial
    // instructions are seen.
    EndOfStream,

    // PartialInstruction - Indicates that the stream ended in the middle of an instruction.
    PartialInstruction,

    // ReadError - Indicates that the underlying stream returned an error.
    ReadError,

    // InvalidInstruction - Generic error for an invalid instruction.
    InvalidInstruction,

    // UnknownOpcode - Indicates that an unrecognized opcode was encountered.
    UnknownOpcode
}
