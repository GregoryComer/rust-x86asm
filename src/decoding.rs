use std::io::{Bytes, Read};
use std::iter::Peekable;
use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mode, Operand, OperandSize, Reg, RegScale, RegType, RoundingMode};
use ::instruction_buffer::*;
use ::instruction_buffer::CompositePrefix; // For disambiguation
use ::instruction_def::*;

pub struct InstructionReader<T: Read> {
    reader: Peekable<Bytes<T>>,
    mode: Mode,
}

impl<T: Read> InstructionReader<T> {
    pub fn new(reader: T, mode: Mode) -> InstructionReader<T> {
        InstructionReader {
            reader: reader.bytes().peekable(),
            mode: mode,
        }
    }

    fn expect_byte(&mut self) -> Result<u8, InstructionDecodingError> {
        match self.reader.next() {
            Some(Ok(b)) => Ok(b),
            Some(Err(_)) => Err(InstructionDecodingError::ReadError),
            None => Err(InstructionDecodingError::PartialInstruction)
        }
    }
    
    #[allow(unused_assignments)] // Compiler wrongly complains about opcode_byte not being read?
    pub fn read(&mut self) -> Result<Instruction, InstructionDecodingError> {
        let mut buffer: InstructionBuffer = Default::default();
        let mut reg_ext = 0; // Extension to mod_rm_reg field
        let mut index_ext = 0; // Extension to sib_index field
        let mut b_ext = 0; // Extension to mod_rm_rm field or sib_base field
        let mut opcode_byte = 0;

        // Check for end of stream
        if self.reader.peek().is_none() {
            return Err(InstructionDecodingError::EndOfStream);
        }

        // Read prefixes
        loop {
            let b = self.expect_byte()?;
            // TODO This could be written without match
            let lookahead: Option<u8> = match self.reader.peek() { 
                Some(&Ok(b)) => Some(b),
                _ => None
            };

            match b {
                PREFIX_LOCK => { buffer.prefix1 = Some(Prefix1::Lock); },
                PREFIX_REPNE => { buffer.prefix1 = Some(Prefix1::RepNE); buffer.f2_prefix = true; },
                PREFIX_REP => { buffer.prefix1 = Some(Prefix1::Rep); buffer.f3_prefix = true; },
                // TODO Remaining rep prefixes?
                PREFIX_OP_SIZE => { buffer.operand_size_prefix = true; },
                PREFIX_ADDR_SIZE => { buffer.address_size_prefix = true; },
                PREFIX_CS => { buffer.prefix2 = Some(Prefix2::CS); },
                PREFIX_SS => { buffer.prefix2 = Some(Prefix2::SS); },
                PREFIX_DS => { buffer.prefix2 = Some(Prefix2::DS); },
                PREFIX_ES => { buffer.prefix2 = Some(Prefix2::ES); },
                PREFIX_FS => { buffer.prefix2 = Some(Prefix2::FS); },
                PREFIX_GS => { buffer.prefix2 = Some(Prefix2::GS); },
                PREFIX_TWO_BYTE_OPCODE => { buffer.is_two_byte_opcode = true; },
                PREFIX_VEX2 => { // Two-byte VEX prefix
                    let data = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::Vex);
                    reg_ext = if data & 0x80 != 0 || self.mode != Mode::Long { 0 } else { 0x8 };
                    buffer.vex_operand = Some((!data >> 3) & if self.mode == Mode::Long { 0xF } else { 0x7 });
                    buffer.vector_len = Some(data & 0x4 != 0);
                    buffer.is_two_byte_opcode = true;
                    buffer.vex_e = Some(false);
                    match data & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.f3_prefix = true; },
                        0x3 => { buffer.f2_prefix = true; },
                        _ => {}
                    }
                },
                PREFIX_VEX3 => { // Three-byte VEX prefix
                    let data1 = self.expect_byte()?;
                    let data2 = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::Vex);
                    reg_ext = if data1 & 0x80 != 0 || self.mode != Mode::Long { 0 } else { 0x8 };
                    index_ext = if data1 & 0x40 != 0 || self.mode != Mode::Long { 0 } else { 0x8 };
                    b_ext = if data1 & 0x20 != 0 || self.mode != Mode::Long { 0 } else { 0x8 };
                    match data1 & 0x1F { // map_select
                        0 => {},
                        1 => buffer.is_two_byte_opcode = true,
                        2 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x38; },
                        3 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x3A; },
                        _ => return Err(InstructionDecodingError::InvalidInstruction)
                    }
                    buffer.vex_e = Some(data2 & 0x80 != 0);
                    buffer.vex_operand = Some((!data2 >> 3) & if self.mode == Mode::Long { 0xF } else { 0x7 });
                    buffer.vector_len = Some(data2 & 0x2 != 0);
                    match data2 & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.f3_prefix = true; },
                        0x3 => { buffer.f2_prefix = true; },
                        _ => {}
                    }
                },
                PREFIX_EVEX if self.mode == Mode::Long || lookahead.map_or(false, |l| l & 0xC0 == 0xC0) => {
                    let data1 = self.expect_byte()?;
                    let data2 = self.expect_byte()?;
                    let data3 = self.expect_byte()?;
                    buffer.composite_prefix = Some(CompositePrefix::Evex);
                    reg_ext |= if data1 & 0x80 == 0 && self.mode == Mode::Long { 0x8 } else { 0 };
                    index_ext |= if data1 & 0x40 == 0 && self.mode == Mode::Long { 0x8 } else { 0 };
                    b_ext |= if data1 & 0x20 == 0 && self.mode == Mode::Long { 0x8 } else { 0 };
                    reg_ext |= if data1 & 0x10 == 0 && self.mode == Mode::Long { 0x10 } else { 0 };
                    match data1 & 0x3 { // map_select
                        0 => {},
                        1 => buffer.is_two_byte_opcode = true,
                        2 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x38; },
                        3 => { buffer.is_two_byte_opcode = true; buffer.primary_opcode = 0x3A; },
                        _ => return Err(InstructionDecodingError::InvalidInstruction)
                    }
                    buffer.vex_e = Some(data2 & 0x80 != 0);
                    buffer.vex_operand = Some((!data2 >> 3) & if self.mode == Mode::Long { 0xF } else { 0x7 });
                    match data2 & 0x3 {
                        0x1 => { buffer.operand_size_prefix = true; },
                        0x2 => { buffer.f3_prefix = true; },
                        0x3 => { buffer.f2_prefix = true; },
                        _ => {}
                    }
                    buffer.merge_mode = Some(if data3 & 0x80 != 0 { MergeMode::Zero } 
                        else { MergeMode::Merge });
                    buffer.vex_l = Some(data3 & 0x40 != 0);
                    buffer.vector_len = Some(data3 & 0x20 != 0);
                    buffer.vex_b = Some(data3 & 0x10 != 0);
                    buffer.mask_reg = Some(data3 & 0x7);
                },
                b if self.mode == Mode::Long && buffer.composite_prefix.is_none() && b & 0xF0 == 0x40 => { // REX prefix
                    buffer.composite_prefix = Some(CompositePrefix::Rex);
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
        let def_res = find_instruction_def_by_opcode(&buffer, self.mode);
            
        // Read a ModR/M if we found a valid def which needs one or if we need one to disambiguate.
        if def_res.map(|def| InstructionReader::<T>::has_mod_rm(def)).unwrap_or(false) ||
            matches!(def_res, Err(FindInstructionDefByOpcodeError::NeedModRm)) {
            let mod_rm = self.expect_byte()?;
            buffer.mod_rm_mod = Some(mod_rm >> 6);
            buffer.mod_rm_reg = Some((mod_rm >> 3) & 0x7 | reg_ext);
            buffer.mod_rm_rm = Some(mod_rm & 0x7);
            
            // SIB
            if InstructionReader::<T>::has_sib(addr_mode, &buffer) {
                let sib = self.expect_byte()?;
                buffer.sib_scale = Some(sib >> 6);
                buffer.sib_index = Some((sib >> 3) & 0x7 | index_ext);
                buffer.sib_base = Some(sib & 0x7 | b_ext);
            } else {
                buffer.mod_rm_reg = buffer.mod_rm_reg.map(|reg| reg | b_ext);
                buffer.mod_rm_rm = buffer.mod_rm_rm.map(|rm| rm | index_ext);
            }
        }
        
        // If we have a def, unwrap it, otherwise try again to find one using the modr/m byte
        // we read.
        let def = def_res.or_else(|_| find_instruction_def_by_opcode(&buffer, self.mode)
            .map_err(|_| InstructionDecodingError::UnknownOpcode))?;

        // Build operands (reading immediates as appropriate)
        // TODO Could re-write this without vec
        let operand_results: Result<Vec<Operand>, InstructionDecodingError> = 
            def.operands.iter().filter_map(
                |maybe_op_def| maybe_op_def.as_ref().map(
                    |op_def| self.read_operand(op_def, &buffer)
            )).collect();
        let mut operands_iter = operand_results?.into_iter();

        Ok(Instruction {
            mnemonic: def.mnemonic,
            operand1: operands_iter.next(),
            operand2: operands_iter.next(),
            operand3: operands_iter.next(),
            operand4: operands_iter.next(),
            lock: buffer.prefix1 == Some(Prefix1::Lock),
            rounding_mode: if def.allow_rounding && buffer.vex_b.unwrap_or(false) &&
                buffer.mod_rm_mod.map_or(false, |m| m == 0b11) {  
                Some(RoundingMode::from_code(
                    buffer.vex_l.map_or(0, |l| if l { 2 } else { 0 }) +
                    buffer.vector_len.map_or(0, |l| if l { 1 } else { 0 })
                ).unwrap())
            } else { None },
            merge_mode: if def.allow_merge_mode && 
                buffer.merge_mode != Some(MergeMode::Merge) { buffer.merge_mode } else { None },
            sae: def.allow_sae && buffer.vex_b.unwrap_or(false) &&
                buffer.mod_rm_mod.map_or(false, |r| r == 0b11),
            mask: if def.allow_mask && buffer.mask_reg != Some(0) {
                buffer.mask_reg.map(|r| MaskReg::from_code(r).unwrap())
            } else { None },
            broadcast: InstructionReader::<T>::get_broadcast(def, &buffer),
        })
    }

    fn read_operand(&mut self, op_def: &OperandDefinition, buffer: &InstructionBuffer)
        -> Result<Operand, InstructionDecodingError> {

        let size = InstructionReader::<T>::get_operand_size(op_def, buffer);
        let addr_size = InstructionReader::<T>::get_address_size(self.mode, buffer);

        // We can assume that we have a ModR/M byte if we've gotten to this point as it
        // would have errored out if we needed one but didn't read one.
        Ok(match op_def.encoding {
            OperandEncoding::ModRmReg =>
                if let OperandType::Reg(reg_type) = op_def.op_type {
                    Operand::Direct(Reg::from_code_reg_type(
                        buffer.mod_rm_reg.unwrap(), reg_type, size, buffer.has_rex())
                        .ok_or(InstructionDecodingError::InvalidInstruction)?)
                } else { panic!("Invalid operand definition."); },

            OperandEncoding::ModRmRm => { // TODO Handle MIB
                let reg_type = if let OperandType::Reg(reg_type) = op_def.op_type { reg_type }
                    else if let OperandType::Set(set) = op_def.op_type {
                        set.iter().filter_map(|i| if let OperandType::Reg(reg_type) = *i 
                            { Some(reg_type) } else { None }).next().unwrap_or(RegType::General)
                    } else { RegType::General };
                self.rm_helper(buffer, op_def, 
                    |c| Reg::from_code_reg_type(c, reg_type, size, buffer.has_rex()))?
            },

            OperandEncoding::Vex =>
                if let OperandType::Reg(reg_type) = op_def.op_type {
                    Operand::Direct(Reg::from_code_reg_type(
                        buffer.vex_operand.unwrap(), reg_type, size, buffer.has_rex())
                        .ok_or(InstructionDecodingError::InvalidInstruction)?)
                } else { panic!("Invalid operand definition."); },

            OperandEncoding::Imm =>
                match op_def.op_type {
                    OperandType::Reg(reg_type) =>
                        Operand::Direct(Reg::from_code_reg_type(
                            self.expect_byte()? >> 4, reg_type, size, buffer.has_rex())
                            .ok_or(InstructionDecodingError::InvalidInstruction)?),
                    OperandType::Imm => match op_def.size {
                        OperandSize::Byte => self.expect_byte().map(|b| Operand::Literal8(b))?,
                        OperandSize::Word => 
                            (0..2).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
                                |b| a | ((b as u16) << (8*n) )))).map(|b| Operand::Literal16(b))?,
                        OperandSize::Dword => 
                            (0..4).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
                                |b| a | ((b as u32) << (8*n) )))).map(|b| Operand::Literal32(b))?,
                        OperandSize::Far16 => { // 16:16
                            let addr = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a|
                                self.expect_byte().map(|b| a | ((b as u16) << (8*n) ))))?;
                            let segment = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a|
                                self.expect_byte().map(|b| a | ((b as u16) << (8*n) ))))?;
                            Operand::MemoryAndSegment16(segment, addr)
                        },
                        OperandSize::Far32 => { // 16:32
                            let addr = (0..4).fold(Ok(0), |acc, n| acc.and_then(|a|
                                self.expect_byte().map(|b| a | ((b as u32) << (8*n) ))))?;
                            let segment = (0..2).fold(Ok(0), |acc, n| acc.and_then(|a|
                                self.expect_byte().map(|b| a | ((b as u16) << (8*n) ))))?;
                            Operand::MemoryAndSegment32(segment, addr)
                        },
                        _ => unimplemented!()
                    },
                    OperandType::Rel(_) => 
                        Operand::Offset(
                            (0..op_def.size.bits() >> 3).fold(Ok(0), |acc, n| acc.and_then(|a|
                                self.expect_byte().map(|b| a | ((b as u64) << (8*n) ))))?,
                    None, None),
                    _ => panic!("Bad instruction definition.")
                },

            OperandEncoding::OpcodeAddend =>
                if let OperandType::Reg(reg_type) = op_def.op_type {
                    Operand::Direct(Reg::from_code_reg_type(
                        buffer.primary_opcode & 0x7, reg_type, size, buffer.has_rex())
                        .ok_or(InstructionDecodingError::InvalidInstruction)?)
                } else { panic!("Invalid operand definition."); },

            OperandEncoding::Offset =>
                Operand::Offset(match addr_size {
                    OperandSize::Word => self.read_disp16()? as u64,
                    OperandSize::Dword => self.read_disp32()? as u64,
                    // OperandSize::Qword => self.read_disp64()? as u64,
                    _ => unimplemented!() // TODO?
                }, Some(op_def.size), None),

            OperandEncoding::Mib => unimplemented!(),

            OperandEncoding::Fixed =>
                match op_def.op_type {
                    OperandType::Fixed(FixedOperand::Reg(reg)) =>
                        Operand::Direct(reg),
                    OperandType::Fixed(FixedOperand::Constant(v)) =>
                        match op_def.size {
                            OperandSize::Unsized |
                            OperandSize::Byte => Operand::Literal8(v as u8),
                            OperandSize::Word => Operand::Literal16(v as u16),
                            OperandSize::Dword => Operand::Literal32(v),
                            OperandSize::Qword => Operand::Literal64(v as u64),
                            _ => panic!("Invalid operand definition.")
                        },
                    _ => panic!("Invalid operand definition.")
                }
        })
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

    #[allow(dead_code)]
    fn read_disp64(&mut self) -> Result<u64, InstructionDecodingError> {
        (0..8).fold(Ok(0), |acc, n| acc.and_then(|a| self.expect_byte().map(
            |b| a | ((b as u64) << (8*n) ))))
    }

    fn has_mod_rm(def: &InstructionDefinition) -> bool {
        def.opcode_ext.is_some() ||
        def.operands.iter().any(|o| o.as_ref().map_or(false, |op| match op.encoding {
            OperandEncoding::ModRmReg |
            OperandEncoding::ModRmRm |
            OperandEncoding::Mib
                => true,
            _ => false
        }))
    }

    fn get_address_size(mode: Mode, buffer: &InstructionBuffer) -> OperandSize {
        match (mode, buffer.address_size_prefix) {
            (Mode::Real, false) | (Mode::Protected, true) => OperandSize::Word,
            (Mode::Real, true) | (Mode::Protected, false) | (Mode::Long, true) => OperandSize::Dword,
            (Mode::Long, false) => OperandSize::Qword
        }
    }

    fn has_sib(mode: Mode, buffer: &InstructionBuffer) -> bool {
        (mode != Mode::Real) && (buffer.mod_rm_mod.and_then(|rm_mod| buffer.mod_rm_rm.map(
            |rm_rm| (rm_rm & 0b111) == 0b100 && rm_mod != 0b11)).unwrap_or(false))
    }

    fn rm_helper<TConv>(&mut self, buffer: &InstructionBuffer, op_def: &OperandDefinition,
        conv_proc: TConv) -> Result<Operand, InstructionDecodingError>
        where TConv : Fn(u8) -> Option<Reg> {
        let rm = buffer.mod_rm_rm.ok_or(InstructionDecodingError::InvalidInstruction)?;
        let addr_size = InstructionReader::<T>::get_address_size(self.mode, buffer);

        Ok(match addr_size {
            OperandSize::Word => {
                let mode = buffer.mod_rm_mod.ok_or(InstructionDecodingError::InvalidInstruction)?;
                let size = InstructionReader::<T>::get_operand_size(op_def, buffer);
                let segment = buffer.get_segment_reg();

                if mode == 0b11 { 
                    return conv_proc(rm).ok_or(InstructionDecodingError::InvalidInstruction)
                        .map(|r| Operand::Direct(r)); 
                }

                let disp = if mode == 0 && rm != 0b110 || mode == 3 { None } // No displacement
                    else { // 8/16-bit displacement
                        Some(if mode == 1 { self.read_disp8()? as u64 } 
                            else { self.read_disp16()? as u64 })
                    };

                let (reg1, reg2) = match rm {
                    0 => (Some(Reg::BX), Some(Reg::SI)),
                    1 => (Some(Reg::BX), Some(Reg::DI)),
                    2 => (Some(Reg::BP), Some(Reg::SI)),
                    3 => (Some(Reg::BP), Some(Reg::DI)),
                    4 => (Some(Reg::SI), None),
                    5 => (Some(Reg::DI), None),
                    6 => if mode == 0 { (None, None) } else { (Some(Reg::BP), None) },
                    7 => (Some(Reg::BX), None),
                    _ => unreachable!()
                };

                match (reg1, reg2, disp) {
                    (None, None, Some(addr)) => Operand::Memory(addr, Some(size), segment),
                    (Some(r1), None, None) => Operand::Indirect(r1, Some(size), segment),
                    (Some(r1), None, Some(disp)) => 
                        Operand::IndirectDisplaced(r1, disp, Some(size), segment),
                    (Some(r1), Some(r2), None) =>
                        Operand::IndirectScaledIndexed(r1, r2, RegScale::One, Some(size), segment),
                    (Some(r1), Some(r2), Some(disp)) =>
                        Operand::IndirectScaledIndexedDisplaced(r1, r2, RegScale::One, disp, 
                            Some(size), segment),
                    _ => unreachable!()
                }
            },
            addr_size @ OperandSize::Dword | addr_size @ OperandSize::Qword => {
                let size = Some(InstructionReader::<T>::get_operand_size(op_def, buffer));
                let segment = buffer.get_segment_reg();
                match buffer.mod_rm_mod.ok_or(InstructionDecodingError::InvalidInstruction)? & 0x7 {
                    0b00 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b011 | 0b110 | 0b111 => // [RM]
                                Operand::Indirect(Reg::from_code_general_sized(rm,
                                    InstructionReader::<T>::has_rex(buffer),
                                    addr_size).ok_or(InstructionDecodingError::InvalidInstruction)?,
                                    size, segment),
                            0b100 => self.sib_helper(buffer, op_def, addr_size)?, // [SIB]
                            0b101 => if addr_size == OperandSize::Dword { Operand::Memory(self.read_disp32()? as u64, size, segment) }
                                        else { Operand::Offset(self.read_disp32()? as u64, size, segment) },
                            _ => unreachable!()
                        }
                    },
                    0b01 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b011 | 0b101 | 0b110 | 0b111 => // [RM + disp8]
                                Operand::IndirectDisplaced(Reg::from_code_general_sized(rm, InstructionReader::<T>::has_rex(buffer),
                                    addr_size).ok_or(InstructionDecodingError::InvalidInstruction)?,
                                    self.read_disp8()? as u64, size, segment),
                            0b100 => self.sib_helper(buffer, op_def, addr_size)?, // [SIB + disp8]
                            _ => unreachable!()
                        }
                    },
                    0b10 => {
                        match rm {
                            0b000 | 0b001 | 0b010 | 0b011 | 0b101 | 0b110 | 0b111 => // [RM + disp32]
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

    fn sib_helper(&mut self, buffer: &InstructionBuffer, op_def: &OperandDefinition, addr_size: OperandSize) -> 
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
        let size = Some(InstructionReader::<T>::get_operand_size(op_def, buffer));
        
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
                let disp = if mode == 0b01 { self.read_disp8()? as u64 } else { self.read_disp32()? as u64 };
                if index_code == 0b100 { // [base + disp8/32]
                    Operand::IndirectDisplaced(base, disp, size, segment)
                } else { // [base + index*s + disp8/32]
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, size, segment)
                }
            },
            _ => unreachable!()
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

    fn get_operand_type<'a>(op_def: &'a OperandDefinition, buffer: &InstructionBuffer) 
        -> &'a OperandType {
        let has_broadcast = match op_def.op_type {
            OperandType::Bcst(_) => true,
            OperandType::Set(items) => items.iter().any(|t| matches!(*t, OperandType::Bcst(_))),
            _ => false
        };

        match op_def.op_type {
            OperandType::Set(set) => {
                set.iter().find(|t| match **t {
                    OperandType::Reg(_) if op_def.encoding == OperandEncoding::ModRmRm
                        => buffer.mod_rm_mod.map_or(false, |r| r == 0b11),
                    OperandType::Bcst(_) 
                        => buffer.vex_b.unwrap_or(false),
                    OperandType::Mem(_)
                        => !has_broadcast || !buffer.vex_b.unwrap_or(false),
                    _ => true
                }).expect("Bad instruction definition.")
            },
            _ => &op_def.op_type
        }
    }

    fn get_operand_size(op_def: &OperandDefinition, buffer: &InstructionBuffer)
        -> OperandSize {
        let op_type = InstructionReader::<T>::get_operand_type(op_def, buffer);
        let s = match *op_type {
            OperandType::Mem(Some(s)) |
            OperandType::Bcst(s) => s,
            _ => op_def.size
        };

        match s {
            OperandSize::Far16 => OperandSize::Dword,
            OperandSize::Far32 => OperandSize::Fword,
            OperandSize::Far64 => OperandSize::Tbyte,
            _ => s
        }
    }

    pub fn has_rex(buffer: &InstructionBuffer) -> bool {
        buffer.composite_prefix.as_ref().map(|p| *p == CompositePrefix::Rex).unwrap_or(false)
    }

    fn get_broadcast(def: &InstructionDefinition, buffer: &InstructionBuffer)
        -> Option<BroadcastMode> {
        buffer.vex_b.map_or(None, |b| if b {
            def.operands.iter().filter_map(|o| o.as_ref().and_then(|op |
                  InstructionReader::<T>::get_broadcast_helper(&op.op_type, op.size))).next()
        } else { None })
    }

    fn get_broadcast_helper(op_type: &OperandType, op_size: OperandSize) -> Option<BroadcastMode> {
        if let OperandType::Bcst(s) = *op_type {
            Some(BroadcastMode::from_multiplier((op_size.bits() / s.bits()) as u8)
                .expect("Bad instruction definition."))
        } else if let OperandType::Set(set) = *op_type {
            set.iter().filter_map(
               |i| InstructionReader::<T>::get_broadcast_helper(i, op_size)).next()
        } else { None }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstructionDecodingError {
    // EndOfStream - Indicates no more bytes are available on the underlying stream.
    // Not returned when partial instructions are seen.
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
