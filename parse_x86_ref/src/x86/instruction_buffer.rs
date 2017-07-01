use std::io::{ Write };
use std::io::Result as IoResult;
use x86::instruction_def::{ InstructionEncodingError };
use byteorder::{ LittleEndian, WriteBytesExt };

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

#[derive(Default, Debug)]
pub struct InstructionBuffer { 
    pub prefix1: Option<Prefix1>,
    pub prefix2: Option<Prefix2>,
    pub operand_size_prefix: bool,
    pub address_size_prefix: bool,
    pub fixed_prefix: Option<u8>,
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

    // TODO Force REX
}

impl InstructionBuffer {
    pub fn write<W>(&self, writer: &mut W) -> Result<usize, InstructionEncodingError> 
        where W: Write {
        self.write_inner(writer).map_err(|_| InstructionEncodingError::WriteFailed)
    }
    
    fn write_inner<W>(&self, writer: &mut W) -> IoResult<usize> 
        where W: Write {
        let mut bytes_written: usize = 0;

        println!("{:?}", self);

        // Prefix 1
        if let Some(p1) = self.get_prefix1_byte() { writer.write_all(&[p1])?; bytes_written += 1; }

        // Prefix 2
        if let Some(p2) = self.get_prefix2_byte() { writer.write_all(&[p2])?; bytes_written += 1; }

        // Operand size prefix
        if self.operand_size_prefix { writer.write_all(&[PREFIX_OP_SIZE])?; bytes_written += 1; }
        
        // Address size prefix
        if self.address_size_prefix { writer.write_all(&[PREFIX_ADDR_SIZE])?; bytes_written += 1; }

        // Fixed prefix
        if let Some(pre) = self.fixed_prefix { writer.write_all(&[pre])?; bytes_written += 1; }

        // Two byte opcode prefix
        if self.is_two_byte_opcode { writer.write_all(&[PREFIX_TWO_BYTE_OPCODE])?; bytes_written += 1; }

        // Primary opcode
        writer.write_all(&[self.primary_opcode + self.opcode_add.unwrap_or(0)])?; bytes_written += 1;

        // Secondary opcode
        if let Some(op) = self.secondary_opcode { writer.write_all(&[op])?; bytes_written += 1; }

        // ModR/M byte
        if let Some(mod_rm) = self.get_mod_rm() { writer.write_all(&[mod_rm])?; bytes_written += 1; }

        // SIB
        if let Some(sib) = self.get_sib() { writer.write_all(&[sib])?; bytes_written += 1; }

        // Immediate values
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

    pub fn add_immediate(&mut self, val: ImmediateValue) {
        if self.immediate.is_none() {
            self.immediate = Some(val);
        } else if self.immediate2.is_none() {
            self.immediate2 = Some(val);
        } else { panic!("To many immediate values."); }
    }

    fn has_mod_rm(&self) -> bool { self.mod_rm_mod.is_some() || self.mod_rm_rm.is_some() || self.mod_rm_reg.is_some() }

    fn get_mod_rm(&self) -> Option<u8> {
        // TODO Handle 64-bit extensions (REX/VEX/EVEX)
        if self.has_mod_rm() {
            Some(self.mod_rm_mod.map(|v| (v & 0x3) << 6).unwrap_or(0) |
                self.mod_rm_reg.map(|v| (v & 0x7) << 3).unwrap_or(0) |
                self.mod_rm_rm.map(|v| v & 0x7).unwrap_or(0))
        } else { None }
    }

    fn has_sib(&self) -> bool { self.sib_scale.is_some() || self.sib_index.is_some() || self.sib_base.is_some() }

    fn get_sib(&self) -> Option<u8> {
        // TODO Handle 64-bit extensions (REX/VEX/EVEX)
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

#[derive(Debug)]
pub enum Prefix1 {
    Lock,
    Rep,
    RepNE, RepNZ,
    RepE,  RepZ
}

#[derive(Debug)]
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
