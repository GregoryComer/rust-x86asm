use std::io::{ Write };
use std::io::Result as IoResult;
use byteorder::{ LittleEndian, WriteBytesExt };
use ::{ InstructionEncodingError, Mode, SegmentReg };
use ::instruction::MergeMode;

pub const PREFIX_LOCK: u8 = 0xF0;
pub const PREFIX_REPNE: u8 = 0xF2; // REPNE/REPNZ
pub const PREFIX_REP: u8 = 0xF3; // REP/REPE/REPZ
pub const PREFIX_OP_SIZE: u8 = 0x66;
pub const PREFIX_ADDR_SIZE: u8 = 0x67;
pub const PREFIX_CS: u8 = 0x2E;
pub const PREFIX_SS: u8 = 0x36;
pub const PREFIX_DS: u8 = 0x3E;
pub const PREFIX_ES: u8 = 0x26;
pub const PREFIX_FS: u8 = 0x64;
pub const PREFIX_GS: u8 = 0x65;
pub const PREFIX_BRANCH_NOT_TAKEN: u8 = 0x2E;
pub const PREFIX_BRANCH_TAKEN: u8 = 0x3E;
pub const PREFIX_TWO_BYTE_OPCODE: u8 = 0x0F;
pub const PREFIX_VEX2: u8 = 0xC5;
pub const PREFIX_VEX3: u8 = 0xC4;
pub const PREFIX_EVEX: u8 = 0x62;
pub const FWAIT: u8 = 0x9B;

#[derive(Debug)]
pub struct InstructionBuffer { 
    pub prefix1: Option<Prefix1>,
    pub prefix2: Option<Prefix2>,
    pub operand_size_prefix: bool,
    pub operand_size_64: bool,
    pub address_size_prefix: bool,
    pub f2_prefix: bool,
    pub f3_prefix: bool,
    pub fwait: bool,
    pub is_two_byte_opcode: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub opcode_add: Option<u8>,
    pub mod_rm_mod: Option<u8>,
    pub mod_rm_reg: Option<u8>,
    pub mod_rm_rm: Option<u8>,
    pub sib_scale: Option<u8>,
    pub sib_index: Option<u8>,
    pub sib_base: Option<u8>,
    pub immediate: Option<ImmediateValue>,
    pub immediate2: Option<ImmediateValue>,
    pub displacement: Option<ImmediateValue>,
    pub vex_operand: Option<u8>,
    pub vex_e: Option<bool>,
    pub vector_len: Option<bool>,
    pub mask_reg: Option<u8>,
    pub merge_mode: Option<MergeMode>,
    pub vex_b: Option<bool>,
    pub vex_l: Option<bool>,
    pub composite_prefix: Option<CompositePrefix>,

    // TODO Force REX
}

impl InstructionBuffer {
    pub fn write<W>(&self, writer: &mut W, mode: Mode) -> Result<usize, InstructionEncodingError> 
        where W: Write {
        self.write_inner(writer, mode).map_err(|_| InstructionEncodingError::WriteFailed)
    }
    
    fn write_inner<W>(&self, writer: &mut W, mode: Mode) -> IoResult<usize> 
        where W: Write {
        let mut bytes_written: usize = 0;

        // TODO Return error if certain vex bits are double used (i.e. b)
        // TODO Support vector sib (See Intel x86 manual - AVX)
        // TODO Support compressed displacement when EVEX is used (see Intel x86 - Vol. 2A 2-39)

        let emit_evex = self.should_emit_evex();
        let emit_vex = !emit_evex && self.should_emit_vex();
        let emit_rex = !emit_evex && !emit_vex && self.should_emit_rex();

        if self.fwait { writer.write_all(&[FWAIT])?; bytes_written += 1; }

        // Prefix 1
        if let Some(p1) = self.get_prefix1_byte() { writer.write_all(&[p1])?; bytes_written += 1; }

        // Prefix 2
        if let Some(p2) = self.get_prefix2_byte() { writer.write_all(&[p2])?; bytes_written += 1; }
        
        // Address size prefix
        if self.address_size_prefix { writer.write_all(&[PREFIX_ADDR_SIZE])?; bytes_written += 1; }

        // Operand size prefix
        if !emit_vex && !emit_evex {
            if self.operand_size_prefix { writer.write_all(&[PREFIX_OP_SIZE])?; bytes_written += 1; }
        }

        // F2/F3
        if !emit_vex && !emit_evex {
            if self.f2_prefix {
                writer.write_all(&[0xF2])?; bytes_written += 1;
            }
            if self.f3_prefix {
                writer.write_all(&[0xF3])?; bytes_written += 1;
            }
        }

        if emit_evex {
            bytes_written += self.write_evex(writer, mode)?;
        } else if emit_vex {
            bytes_written += self.write_vex(writer)?;
        } else if emit_rex {
            bytes_written += self.write_rex(writer)?;
        }

        // Two byte opcode prefix
        if self.is_two_byte_opcode && !emit_vex && !emit_evex {
            writer.write_all(&[PREFIX_TWO_BYTE_OPCODE])?; bytes_written += 1; 
        }

        // Primary opcode
        if !(emit_vex || emit_evex) || !(self.primary_opcode == 0x38 || self.primary_opcode == 0x3A) {
            writer.write_all(&[self.primary_opcode + self.opcode_add.unwrap_or(0)])?; bytes_written += 1;
        }

        // Secondary opcode
        if let Some(op) = self.secondary_opcode { writer.write_all(&[op])?; bytes_written += 1; }

        // ModR/M byte
        if let Some(mod_rm) = self.get_mod_rm() { writer.write_all(&[mod_rm])?; bytes_written += 1; }

        // SIB
        if let Some(sib) = self.get_sib() { writer.write_all(&[sib])?; bytes_written += 1; }

        // Immediate values
        if let Some(ref v) = self.displacement {
            bytes_written += InstructionBuffer::write_immediate(writer, &v)?;
        }
        if let Some(ref v) = self.immediate {
            bytes_written += InstructionBuffer::write_immediate(writer, &v)?;
        }
        if let Some(ref v) = self.immediate2 {
            bytes_written += InstructionBuffer::write_immediate(writer, &v)?;
        }

        Ok(bytes_written)
    }

    fn write_immediate<W>(writer: &mut W, val: &ImmediateValue) -> IoResult<usize> 
        where W: Write {
        match *val {
            ImmediateValue::MemoryAndSegment16(seg, val) => {
                writer.write_u16::<LittleEndian>(val)?;
                writer.write_u16::<LittleEndian>(seg)?;
                Ok(4)
            }
            ImmediateValue::MemoryAndSegment32(seg, val) => {
                writer.write_u32::<LittleEndian>(val)?;
                writer.write_u16::<LittleEndian>(seg)?;
                Ok(6)
            },
            ImmediateValue::Displacement8(val) | 
            ImmediateValue::Literal8(val) => {
                writer.write_u8(val)?; Ok(1)
            },
            ImmediateValue::Literal16(val) => {
                writer.write_u16::<LittleEndian>(val)?; Ok(2)
            },
            ImmediateValue::Displacement32(val) | 
            ImmediateValue::Literal32(val) => {
                writer.write_u32::<LittleEndian>(val)?; Ok(4)
            },
            ImmediateValue::Literal64(val) => {
                writer.write_u64::<LittleEndian>(val)?; Ok(8)
            }
        }
    }

    fn write_rex<W>(&self, writer: &mut W) -> IoResult<usize>
        where W: Write {
        let rex_byte = 0x40 |
            if self.operand_size_64 { 1 << 3 } else { 0 } |
            self.mod_rm_reg.map(|reg| (reg & 0x8) >> 1).unwrap_or(0) |
            self.sib_index.map(|idx| (idx & 0x8) >> 2).unwrap_or(0) |
            self.mod_rm_rm.map(|rm| (rm & 0x8) >> 3)
                .or(self.sib_base.map(|b| b & 0x8)).unwrap_or(0);
        writer.write(&[rex_byte])
    }

    fn write_vex<W>(&self, writer: &mut W) -> IoResult<usize>
        where W: Write {
        let vex_r = self.mod_rm_reg.map(|r| (!r & 0x8) >> 3).unwrap_or(0);
        let vex_x = self.sib_index.map(|s| (!s & 0x8) >> 3);
        let vex_b = self.mod_rm_rm.or(self.sib_base).map(|r| (!r & 0x8) >> 3);
        let vex_we = if self.vex_e.unwrap_or(self.operand_size_64) { 1 } else { 0 };
        let pp = if self.operand_size_prefix { 1 }
                 else if self.f3_prefix { 2 }
                 else if self.f2_prefix { 3 }
                 else { 0 };
        let map_select = if self.primary_opcode == 0x38 { 2 }
                         else if self.primary_opcode == 0x3A { 3 }
                         else { 1 };

        // Select 2 or 3-byte form
        if vex_x.map(|x| x == 1).unwrap_or(true) &&
            vex_b.map(|b| b == 1).unwrap_or(true) &&
            (vex_we == 0) &&
            (map_select == 1) { // 2-byte form

            let b2 = vex_r << 7 |
                     (self.vex_operand.map(|v| (!v & 0xF)).unwrap_or(0xF) << 3) |
                     if self.vector_len.unwrap_or(false) { 1 << 2 } else { 0 } |
                     pp;

            writer.write(&[PREFIX_VEX2, b2])
        } else { // 3-byte form
            let b2 = vex_r << 7 |
                     vex_x.unwrap_or(1) << 6 |
                     vex_b.unwrap_or(1) << 5 |
                     map_select;

            let b3 = vex_we << 7 |
                     (self.vex_operand.map(|v| (!v & 0xF)).unwrap_or(0xF) << 3) |
                     if self.vector_len.unwrap_or(false) { 1 << 2 } else { 0 } |
                     pp;

            writer.write(&[PREFIX_VEX3, b2, b3])
        }
    }

    fn write_evex<W>(&self, writer: &mut W, mode: Mode) -> IoResult<usize>
        where W: Write {
        let vex_operand = self.vex_operand.map(|v| 0x1F - v);
        let vex_r = if mode == Mode::Long { self.mod_rm_reg.map(|r| if r & 0x8 == 0 { 1 } else { 0 }).unwrap_or(0) } else { 1 };
        let vex_r2 = if mode == Mode::Long { self.mod_rm_reg.map(|r| if r & 0x10 == 0 { 1 } else { 0 }).unwrap_or(0) } else { 1 };
        let vex_x = if mode == Mode::Long { self.sib_index.map(|s| if s & 0x8 == 0 { 1 } else { 0 })
            .or(self.mod_rm_rm.map(|r| if r & 0x10 == 0 { 1 } else { 0 })).unwrap_or(1) } else { 1 };
        let vex_b = if mode == Mode::Long { self.mod_rm_rm.or(self.sib_base).map(|r| if r & 0x8 == 0 { 1 } else { 0 }).unwrap_or(1) } else { 1 };
        let vex_b2 = if self.vex_b.unwrap_or(false) { 1 } else { 0 };
        let vex_v = vex_operand.map(|s| (s & 0x10) >> 4).unwrap_or(1);
        let vex_v4 = vex_operand.map(|s| s & 0xF).unwrap_or(0xF);
        let vex_we = if self.vex_e.unwrap_or(false) { 1 } else { 0 }; // if self.op_size_64_behavior == OpSize64Behavior::Force64EvexOnly { 1 } else { self.vex_e.or(Some(self.operand_size_64)).map(|b| if b { 1 } else { 0 }).unwrap_or(0) }; // TODO
        let vex_l = self.vector_len.map(|v| if v { 1 } else { 0 }).unwrap_or(0);
        let vex_l2 = if self.vex_l.unwrap_or(false) { 1 } else { 0 };
        let vex_a3 = self.mask_reg.map(|m| m & 0x7).unwrap_or(0);
        let vex_z = self.merge_mode.map(|m| match m {
            MergeMode::Merge => 0,
            MergeMode::Zero => 1
        }).unwrap_or(0);
        let pp = if self.operand_size_prefix { 1 }
             else if self.f3_prefix { 2 }
             else if self.f2_prefix { 3 }
             else { 0 };
        let map_select = if self.primary_opcode == 0x38 { 2 }
                     else if self.primary_opcode == 0x3A { 3 }
                     else { 1 };

        let b2 = vex_r << 7 |
                 vex_x << 6 |
                 vex_b << 5 |
                 vex_r2 << 4 |
                 map_select & 0x3;

        let b3 = vex_we << 7 |
                 vex_v4 << 3 |
                 4 |
                 pp;

        let b4 = vex_z << 7|
                 vex_l2 << 6 | 
                 vex_l << 5 |
                 vex_b2 << 4 |
                 vex_v << 3 |
                 vex_a3;

        writer.write(&[PREFIX_EVEX, b2, b3, b4])
    }

    pub fn add_immediate(&mut self, val: ImmediateValue) {
        if self.immediate.is_none() {
            self.immediate = Some(val);
        } else if self.immediate2.is_none() {
            self.immediate2 = Some(val);
        } else {
            panic!("Too many immediate values.");
        }
    }

    fn has_mod_rm(&self) -> bool { self.mod_rm_mod.is_some() || self.mod_rm_rm.is_some() || self.mod_rm_reg.is_some() }

    fn get_mod_rm(&self) -> Option<u8> {
        if self.has_mod_rm() {
            Some(self.mod_rm_mod.map(|v| (v & 0x3) << 6).unwrap_or(0) |
                self.mod_rm_reg.map(|v| (v & 0x7) << 3).unwrap_or(0) |
                self.mod_rm_rm.map(|v| v & 0x7).unwrap_or(0))
        } else { None }
    }

    fn has_sib(&self) -> bool { self.sib_scale.is_some() || self.sib_index.is_some() || self.sib_base.is_some() }

    fn get_sib(&self) -> Option<u8> {
        if self.has_sib() {
            Some(self.sib_scale.map(|v| (v & 0x3) << 6).unwrap_or(0) |
                self.sib_index.map(|v| (v & 0x7) << 3).unwrap_or(0) |
                self.sib_base.map(|v| v & 0x7).unwrap_or(0))
        } else { None }

    }

    fn get_prefix1_byte(&self) -> Option<u8> {
        self.prefix1.as_ref().map(|p1| match *p1 {
            Prefix1::Lock  => PREFIX_LOCK,
            Prefix1::RepNE |
            Prefix1::RepNZ => PREFIX_REPNE,
            Prefix1::Rep   |
            Prefix1::RepE  |
            Prefix1::RepZ  => PREFIX_REP
        })
    }

    fn get_prefix2_byte(&self) -> Option<u8> {
        self.prefix2.as_ref().map(|p2| match *p2 {
            Prefix2::CS => PREFIX_CS,
            Prefix2::DS => PREFIX_DS,
            Prefix2::ES => PREFIX_ES,
            Prefix2::FS => PREFIX_FS,
            Prefix2::GS => PREFIX_GS,
            Prefix2::SS => PREFIX_SS,
            Prefix2::BranchNotTaken => PREFIX_BRANCH_NOT_TAKEN,
            Prefix2::BranchTaken => PREFIX_BRANCH_TAKEN
        })
    }

    fn should_emit_rex(&self) -> bool {
        // Emit a REX prefix if 64-bit operand size is needed, or if reg/index/rm/base
        // need a fourth bit
        self.composite_prefix == Some(CompositePrefix::Rex) ||
        self.operand_size_64 || 
        self.mod_rm_reg.map(|reg| reg & 0x8 != 0).unwrap_or(false) ||
        self.sib_index.map(|inx| inx & 0x8 != 0).unwrap_or(false) ||
        self.mod_rm_rm.map(|rm| rm & 0x8 != 0).unwrap_or(false) ||
        self.sib_base.map(|b| b & 0x8 != 0).unwrap_or(false)
    }

    fn should_emit_vex(&self) -> bool {
        // Emit a VEX prefix if there's a vex operand, vector length override, e bit,
        // or 64-bit operand size override.
        self.composite_prefix == Some(CompositePrefix::Vex) ||
        self.vex_e.is_some() || 
        self.vex_operand.is_some() ||
        self.vector_len.unwrap_or(false)
    }

    fn should_emit_evex(&self) -> bool {
        // Emit a VEX prefix if index needs a fourth bit, reg/rm need a fifth bit,
        // there's an opcode map, extra opcode, vector length override, e bit, or 64-bit
        // operand size override.
        self.composite_prefix == Some(CompositePrefix::Evex) ||
        self.mod_rm_reg.map(|reg| reg & 0x10 != 0).unwrap_or(false) ||
        self.mod_rm_rm.map(|rm| rm & 0x10 != 0).unwrap_or(false) ||
        self.mask_reg.is_some() ||
        self.merge_mode.is_some() ||
        self.vex_b.is_some() ||
        self.vex_l.unwrap_or(false) // TODO Rounding modes?
    }

    pub fn set_segment_override(&mut self, reg: SegmentReg) {
        self.prefix2 = Some(match reg {
            SegmentReg::CS => Prefix2::CS,
            SegmentReg::DS => Prefix2::DS,
            SegmentReg::ES => Prefix2::ES,
            SegmentReg::FS => Prefix2::FS,
            SegmentReg::GS => Prefix2::GS,
            SegmentReg::SS => Prefix2::SS,
        });
    }

    pub fn get_segment_reg(&self) -> Option<SegmentReg> {
        self.prefix2.and_then(|prefix| match prefix {
            Prefix2::CS => Some(SegmentReg::CS),
            Prefix2::DS => Some(SegmentReg::DS),
            Prefix2::ES => Some(SegmentReg::ES),
            Prefix2::FS => Some(SegmentReg::FS),  
            Prefix2::GS => Some(SegmentReg::GS),
            Prefix2::SS => Some(SegmentReg::SS),
            _ => None
        })
    }

    pub fn has_rex(&self) -> bool {
        self.composite_prefix == Some(CompositePrefix::Rex)
    }
}

impl Default for InstructionBuffer {
    fn default() -> InstructionBuffer {
        InstructionBuffer {
            prefix1: None,
            prefix2: None,
            composite_prefix: None,
            operand_size_prefix: false,
            operand_size_64: false,
            address_size_prefix: false,
            f2_prefix: false,
            f3_prefix: false,
            fwait: false,
            is_two_byte_opcode: false,
            primary_opcode: 0,
            secondary_opcode: None,
            opcode_add: None,
            mod_rm_mod: None,
            mod_rm_reg: None,
            mod_rm_rm: None,
            sib_scale: None,
            sib_index: None,
            sib_base: None,
            immediate: None,
            immediate2: None,
            displacement: None,
            vex_operand: None,
            vex_e: None,
            vector_len: None,
            mask_reg: None,
            merge_mode: None,
            vex_b: None,
            vex_l: None,
        }
    }
}

#[derive(Debug)]
pub enum ImmediateValue {
    MemoryAndSegment16(u16, u16),
    MemoryAndSegment32(u16, u32),
    Displacement8(u8),
    Displacement32(u32),
    Literal8(u8),
    Literal16(u16),
    Literal32(u32),
    Literal64(u64)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Prefix1 {
    Lock,
    Rep,
    RepNE, RepNZ,
    RepE,  RepZ
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Prefix2 {
    CS,
    SS,
    DS,
    ES,
    FS,
    GS,
    BranchNotTaken,
    BranchTaken,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CompositePrefix {
    Rex,
    Vex,
    Evex
}
