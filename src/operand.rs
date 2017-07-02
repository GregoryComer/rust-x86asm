use ::instruction::{BroadcastMode, MaskReg, MergeMode, Reg, RegScale, SegmentReg};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operand {
   Direct(Reg),
   Indirect(Reg, Option<OperandSize>, Option<SegmentReg>),
   IndirectDisplaced(Reg, u64, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledIndexed(Reg, Reg, RegScale, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledIndexedDisplaced(Reg, Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>),
   IndirectScaledDisplaced(Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>),
   Memory(u64, Option<OperandSize>, Option<SegmentReg>),
   Literal8(u8),
   Literal16(u16),
   Literal32(u32),
   Literal64(u64),
   MemoryAndSegment16(u16, u16),
   MemoryAndSegment32(u16, u32),
   AVXDestination(Reg, Option<MaskReg>, Option<MergeMode>),
   AVXDestinationIndirect(Reg, Option<OperandSize>, Option<SegmentReg>, Option<MaskReg>, Option<MergeMode>),
   AVXDestinationIndirectDisplaced(Reg, u64, Option<OperandSize>, Option<SegmentReg>, Option<MaskReg>, Option<MergeMode>),
   AVXDestinationIndirectScaledIndexed(Reg, Reg, RegScale, Option<OperandSize>, Option<SegmentReg>, Option<MaskReg>, Option<MergeMode>),
   AVXDestinationIndirectScaledIndexedDisplaced(Reg, Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>, Option<MaskReg>, Option<MergeMode>),
   AVXDestinationIndirectScaledDisplaced(Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>, Option<MaskReg>, Option<MergeMode>),
   AVXBroadcastIndirect(BroadcastMode, Reg, Option<OperandSize>, Option<SegmentReg>),
   AVXBroadcastIndirectDisplaced(BroadcastMode, Reg, u64, Option<OperandSize>, Option<SegmentReg>),
   AVXBroadcastIndirectScaledIndexed(BroadcastMode, Reg, Reg, RegScale, Option<OperandSize>, Option<SegmentReg>),
   AVXBroadcastIndirectScaledIndexedDisplaced(BroadcastMode, Reg, Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>),
   AVXBroadcastIndirectScaledDisplaced(BroadcastMode, Reg, RegScale, u64, Option<OperandSize>, Option<SegmentReg>)
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
            Operand::Memory(_, size, _)
                => size,
            Operand::Literal8(_) => Some(OperandSize::Byte),
            Operand::Literal16(_) => Some(OperandSize::Word),
            Operand::Literal32(_) => Some(OperandSize::Dword),
            Operand::Literal64(_) => Some(OperandSize::Qword),
            Operand::MemoryAndSegment16(..) => Some(OperandSize::Dword),
            Operand::MemoryAndSegment32(..) => Some(OperandSize::Fword),
            Operand::AVXDestination(reg, ..) => Some(reg.size()),
            Operand::AVXDestinationIndirect(_, size, ..) |
            Operand::AVXDestinationIndirectDisplaced(_, _, size, ..) |
            Operand::AVXDestinationIndirectScaledIndexed(_, _, _, size, ..) |
            Operand::AVXDestinationIndirectScaledIndexedDisplaced(_, _, _, _, size, ..) |
            Operand::AVXDestinationIndirectScaledDisplaced(_, _, _, size, ..) |
            Operand::AVXBroadcastIndirect(_, _, size, ..) |
            Operand::AVXBroadcastIndirectDisplaced(_, _, _, size, ..) |
            Operand::AVXBroadcastIndirectScaledIndexed(_, _, _, _, size, ..) |
            Operand::AVXBroadcastIndirectScaledIndexedDisplaced(_, _, _, _, _, size, ..) |
            Operand::AVXBroadcastIndirectScaledDisplaced(_, _, _, _, size, ..)
                => size
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
           Operand::AVXDestinationIndirect(_, _, seg, ..) |
           Operand::AVXDestinationIndirectDisplaced(_, _, _, seg, ..) |
           Operand::AVXDestinationIndirectScaledIndexed(_, _, _, _, seg, ..) |
           Operand::AVXDestinationIndirectScaledIndexedDisplaced(_, _, _, _, _, seg, ..) |
           Operand::AVXDestinationIndirectScaledDisplaced(_, _, _, _, seg, ..) |
           Operand::AVXBroadcastIndirect(_, _, _, seg) |
           Operand::AVXBroadcastIndirectDisplaced(_, _, _, _, seg) |
           Operand::AVXBroadcastIndirectScaledIndexed(_, _, _, _, _, seg) |
           Operand::AVXBroadcastIndirectScaledIndexedDisplaced(_, _, _, _, _, _, seg) |
           Operand::AVXBroadcastIndirectScaledDisplaced(_, _, _, _, _, seg)
                => seg,
            _ => None
        }
    }

    pub fn is_direct(&self) -> bool {
        match *self {
            Operand::Direct(..) |
            Operand::AVXDestination(..) => true,
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
            Operand::AVXBroadcastIndirect(..) |
            Operand::AVXBroadcastIndirectDisplaced(..) |
            Operand::AVXBroadcastIndirectScaledIndexed(..) |
            Operand::AVXBroadcastIndirectScaledIndexedDisplaced(..) |
            Operand::AVXBroadcastIndirectScaledDisplaced(..) |
            Operand::AVXDestinationIndirect(..) |
            Operand::AVXDestinationIndirectDisplaced(..) |
            Operand::AVXDestinationIndirectScaledIndexed(..) |
            Operand::AVXDestinationIndirectScaledIndexedDisplaced(..) |
            Operand::AVXDestinationIndirectScaledDisplaced(..) => true,
            _ => false
        }
    }

    pub fn is_fixed_memory(&self) -> bool {
        match *self {
            Operand::Memory(..) |
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
            Operand::AVXDestination(..) |
            Operand::AVXDestinationIndirect(..) |
            Operand::AVXDestinationIndirectDisplaced(..) |
            Operand::AVXDestinationIndirectScaledIndexed(..) |
            Operand::AVXDestinationIndirectScaledIndexedDisplaced(..) |
            Operand::AVXDestinationIndirectScaledDisplaced(..) |
            Operand::AVXBroadcastIndirect(..) |
            Operand::AVXBroadcastIndirectDisplaced(..) |
            Operand::AVXBroadcastIndirectScaledIndexed(..) |
            Operand::AVXBroadcastIndirectScaledIndexedDisplaced(..) |
            Operand::AVXBroadcastIndirectScaledDisplaced(..) => true,
            _ => false
        }
    }

    pub fn is_avx_op1(&self) -> bool {
        match *self {
            Operand::AVXDestination(..) |
            Operand::AVXDestinationIndirect(..) |
            Operand::AVXDestinationIndirectDisplaced(..) |
            Operand::AVXDestinationIndirectScaledIndexed(..) |
            Operand::AVXDestinationIndirectScaledIndexedDisplaced(..) |
            Operand::AVXDestinationIndirectScaledDisplaced(..) => true,
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

    pub fn get_broadcast_mode(&self) -> Option<BroadcastMode> {
        match *self {
            Operand::AVXBroadcastIndirect(b_mode, ..) |
            Operand::AVXBroadcastIndirectDisplaced(b_mode, ..) |
            Operand::AVXBroadcastIndirectScaledIndexed(b_mode, ..) |
            Operand::AVXBroadcastIndirectScaledIndexedDisplaced(b_mode, ..) |
            Operand::AVXBroadcastIndirectScaledDisplaced(b_mode, ..) => Some(b_mode),
            _ => None
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
    XMMword,    // 128-bit
    YMMword,    // 256-bit
    ZMMword,    // 512-bit
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
            OperandSize::XMMword => 128,
            OperandSize::YMMword => 256,
            OperandSize::ZMMword => 512,
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
            128 => OperandSize::XMMword,
            256 => OperandSize::YMMword,
            512 => OperandSize::ZMMword,
            0 => OperandSize::Unsized,
            _ => return None
        })
    }
}
