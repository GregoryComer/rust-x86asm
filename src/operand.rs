use ::instruction::{Reg, RegScale, SegmentReg};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Operand {
   Direct(Reg),
   Indirect(Reg, Option<OperandSize>, Option<SegmentReg>),
   IndirectDisplaced(Reg, u64, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledIndexed(Reg, Reg, RegScale, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledIndexedDisplaced(Reg, Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledDisplaced(Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>),
   Memory(u64, Option<OperandSize>, Option<SegmentReg>),
   Offset(u64, Option<OperandSize>, Option<SegmentReg>),
   Literal8(u8),
   Literal16(u16),
   Literal32(u32),
   Literal64(u64),
   MemoryAndSegment16(u16, u16),
   MemoryAndSegment32(u16, u32),
}

impl Operand {
    pub fn size(&self) -> Option<OperandSize> {
        match *self {
            Operand::Direct(reg) => Some(reg.size()),
            Operand::Indirect(_, size, _) |
            Operand::IndirectDisplaced(_, _, size, _) |
            Operand::IndirectScaledIndexed(_, _, _, size, _) |
            Operand::IndirectScaledIndexedDisplaced(_, _, _, _, size, _) |
            Operand::IndirectScaledDisplaced(_, _, _, size, _) |
            Operand::Memory(_, size, _) |
            Operand::Offset(_, size, _)
                => size,
            Operand::Literal8(_) => Some(OperandSize::Byte),
            Operand::Literal16(_) => Some(OperandSize::Word),
            Operand::Literal32(_) => Some(OperandSize::Dword),
            Operand::Literal64(_) => Some(OperandSize::Qword),
            Operand::MemoryAndSegment16(..) |
            Operand::MemoryAndSegment32(..)
                => None // TODO?
        }
    }

    pub fn segment_reg(&self) -> Option<SegmentReg> {
        match *self {
           Operand::Indirect(_, _, seg) |
           Operand::IndirectDisplaced(_, _, _, seg) |
           Operand::IndirectScaledIndexed(_, _, _, _, seg) |
           Operand::IndirectScaledIndexedDisplaced(_, _, _, _, _, seg) |
           Operand::IndirectScaledDisplaced(_, _, _, _, seg) |
           Operand::Memory(_, _, seg) |
           Operand::Offset(_, _, seg)
                => seg,
            _ => None
        }
    }

    pub fn is_direct(&self) -> bool {
        match *self {
            Operand::Direct(..) => true,
            _ => false
        }
    }

    pub fn is_memory(&self) -> bool {
        match *self {
            Operand::Indirect(..) |
            Operand::IndirectDisplaced(..) |
            Operand::IndirectScaledIndexed(..) |
            Operand::IndirectScaledIndexedDisplaced(..) |
            Operand::IndirectScaledDisplaced(..) |
            Operand::Memory(..) |
            Operand::Offset(..) => true,
            _ => false
        }
    }

    pub fn is_fixed_memory(&self) -> bool {
        match *self {
            Operand::Memory(..) |
            Operand::Offset(..) |
            Operand::MemoryAndSegment16(..) => true,
            Operand::MemoryAndSegment32(..) => true,
            _ => false
        }
    }

    pub fn is_literal(&self) -> bool {
        match *self {
            Operand::Literal8(_) |
            Operand::Literal16(_) |
            Operand::Literal32(_) |
            Operand::Literal64(_) => true,
            _ => false
        }
    }

    pub fn is_general(&self) -> bool {
        match *self {
           Operand::Direct(r) => r.is_general(),
           _ => false
        }
    }

    pub fn is_fpu(&self) -> bool {
        match *self {
            Operand::Direct(reg) => reg.is_fpu(),
            _ => false
        }
    }

    pub fn is_flags(&self) -> bool {
        match *self {
            Operand::Direct(reg) => reg.is_flags(),
            _ => false
        }
    }
    
    pub fn is_mmx(&self) -> bool {
        match *self {
            Operand::Direct(reg) => reg.is_mmx(),
            _ => false
        }
    }
    
    pub fn is_sse(&self) -> bool {
        match *self {
            Operand::Direct(reg) => reg.is_sse(),
            _ => false
        }
    }
    
    pub fn is_avx(&self) -> bool {
        match *self {
            Operand::Direct(reg) => reg.is_avx(),
            _ => false
        }
    }

    pub fn is_far_pointer(&self) -> bool {
        match *self {
            Operand::MemoryAndSegment16(..) |
            Operand::MemoryAndSegment32(..) => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperandSize {
    Byte,       // 8-bit
    Word,       // 16-bit
    Dword,      // 32-bit
    Fword,      // 48-bit
    Qword,      // 64-bit
    Tbyte,      // 80-bit
    Xmmword,    // 128-bit
    Ymmword,    // 256-bit
    Zmmword,    // 512-bit
    Far16,      // 16:16
    Far32,      // 16:32
    Far64,      // 16:64
    Unsized
}

impl OperandSize {
    pub fn bits(&self) -> u32 {
        match *self {
            OperandSize::Byte => 8,
            OperandSize::Word => 16,
            OperandSize::Dword => 32,
            OperandSize::Fword => 48,
            OperandSize::Qword => 64,
            OperandSize::Tbyte => 80,
            OperandSize::Xmmword => 128,
            OperandSize::Ymmword => 256,
            OperandSize::Zmmword => 512,
            OperandSize::Far16 => 32,
            OperandSize::Far32 => 48,
            OperandSize::Far64 => 80,
            OperandSize::Unsized => 0, // TODO?
        }
    }

    pub fn from_bits(bits: u32) -> Option<OperandSize> {
        Some(match bits {
            8 => OperandSize::Byte,
            16 => OperandSize::Word,
            32 => OperandSize::Dword,
            48 => OperandSize::Fword,
            64 => OperandSize::Qword,
            80 => OperandSize::Tbyte,
            128 => OperandSize::Xmmword,
            256 => OperandSize::Ymmword,
            512 => OperandSize::Zmmword,
            0 => OperandSize::Unsized,
            _ => return None
        })
    }
}
