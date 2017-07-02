use std::io::Write;
use ::{Mnemonic, Mode, ProcessorLevel, InstructionEncodingError};
use ::instruction_def::{InstructionDefinition, find_instruction_def, OperandType};
use ::operand::{Operand, OperandSize};

pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
    pub operand3: Option<Operand>,
    pub operand4: Option<Operand>,
    pub flags: InstructionFlags,
}

impl Instruction {
    pub fn encode<W>(&self, writer: &mut W, mode: Mode, proc_level: ProcessorLevel) -> Result<usize, InstructionEncodingError> 
        where W: Write {
        let encoding = find_instruction_def(&self, mode, proc_level)?;
        encoding.encode(writer, &self, mode, proc_level)
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            mnemonic: Mnemonic::ADD, // Shouldn't ever need a default mnemonic, but compiler requires it.
            operand1: None,
            operand2: None,
            operand3: None,
            operand4: None,
            flags: Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoundingMode {
    Nearest,
    Down,
    Up,
    Zero
}

impl RoundingMode {
    pub fn get_code(&self) -> u8 {
        match *self {
            RoundingMode::Nearest => 0,
            RoundingMode::Down => 1,
            RoundingMode::Up => 2,
            RoundingMode::Zero => 3
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionFlags {
    pub lock: bool,
    pub rounding_mode: Option<RoundingMode>,
    pub sae: bool
    // TODO Rep prefixes
}

impl Default for InstructionFlags { 
    fn default() -> InstructionFlags { InstructionFlags { 
            lock: false,
            rounding_mode: None,
            sae: false
        }
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RegScale {
    One,
    Two,
    Four,
    Eight
}

impl RegScale {
    pub fn get_sib_code(&self) -> u8 {
       match *self {
           RegScale::One => 0b00,
           RegScale::Two => 0b01,
           RegScale::Four => 0b10,
           RegScale::Eight => 0b11
       }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SegmentReg {
    CS,
    DS,
    ES,
    FS,
    GS,
    SS
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Reg {
    AL, AH, AX, EAX, RAX,
    BL, BH, BX, EBX, RBX,
    CL, CH, CX, ECX, RCX,
    DL, DH, DX, EDX, RDX,
    BP, EBP, RBP,
    SP, ESP, RSP,
    SI, ESI, RSI,
    DI, EDI, RDI,
    IP, EIP, RIP,
    R8, R8D, R8W, R8B,
    R9, R9D, R9W, R9B,
    R10, R10D, R10W, R10B,
    R11, R11D, R11W, R11B,
    R12, R12D, R12W, R12B,
    R13, R13D, R13W, R13B,
    R14, R14D, R14W, R14B,
    R15, R15D, R15W, R15B,
    FLAGS, EFLAGS, RFLAGS,
    CS, DS, ES, FS, GS, SS,
    ST, ST0, ST1, ST2, ST3,
    ST4, ST5, ST6, ST7,
    MM0, MM1, MM2, MM3,
    MM4, MM5, MM6, MM7,
    CR0, CR1, CR2, CR3,
    CR4, CR8,
    DR0, DR1, DR2, DR3,
    DR4, DR5, DR6, DR7,
    TR3, TR4, TR5, TR6, TR7,
    XMM0 , YMM0 , ZMM0 ,
    XMM1 , YMM1 , ZMM1 ,
    XMM2 , YMM2 , ZMM2 ,
    XMM3 , YMM3 , ZMM3 ,
    XMM4 , YMM4 , ZMM4 ,
    XMM5 , YMM5 , ZMM5 ,
    XMM6 , YMM6 , ZMM6 ,
    XMM7 , YMM7 , ZMM7 ,
    XMM8 , YMM8 , ZMM8 ,
    XMM9 , YMM9 , ZMM9 ,
    XMM10, YMM10, ZMM10,
    XMM11, YMM11, ZMM11,
    XMM12, YMM12, ZMM12,
    XMM13, YMM13, ZMM13,
    XMM14, YMM14, ZMM14,
    XMM15, YMM15, ZMM15,
    XMM16, YMM16, ZMM16,
    XMM17, YMM17, ZMM17,
    XMM18, YMM18, ZMM18,
    XMM19, YMM19, ZMM19,
    XMM20, YMM20, ZMM20,
    XMM21, YMM21, ZMM21,
    XMM22, YMM22, ZMM22,
    XMM23, YMM23, ZMM23,
    XMM24, YMM24, ZMM24,
    XMM25, YMM25, ZMM25,
    XMM26, YMM26, ZMM26,
    XMM27, YMM27, ZMM27,
    XMM28, YMM28, ZMM28,
    XMM29, YMM29, ZMM29,
    XMM30, YMM30, ZMM30,
    XMM31, YMM31, ZMM31,
    GDTR, LDTR, IDTR, TR,
    XCR, MSR, PMC,
    K0, K1, K2, K3,
    K4, K5, K6, K7,
    BND0, BND1, BND2, BND3
}

impl Reg {
    pub fn is_64_only(&self) -> bool {
       match *self {
            Reg::RAX | Reg::RBX  | Reg::RCX  | Reg::RDX  |
            Reg::RBP | Reg::RSP  | Reg::RSI  | Reg::RDI  |
            Reg::R8  | Reg::R8D  | Reg::R8W  | Reg::R8B  |
            Reg::R9  | Reg::R9D  | Reg::R9W  | Reg::R9B  |
            Reg::R10 | Reg::R10D | Reg::R10W | Reg::R10B |
            Reg::R11 | Reg::R11D | Reg::R11W | Reg::R11B |
            Reg::R12 | Reg::R12D | Reg::R12W | Reg::R12B |
            Reg::R13 | Reg::R13D | Reg::R13W | Reg::R13B |
            Reg::R14 | Reg::R14D | Reg::R14W | Reg::R14B |
            Reg::R15 | Reg::R15D | Reg::R15W | Reg::R15B |
            Reg::RFLAGS | Reg::RIP |
            Reg::CR8 |
            Reg::YMM0  | Reg::ZMM0  | 
            Reg::YMM1  | Reg::ZMM1  |
            Reg::YMM2  | Reg::ZMM2  |
            Reg::YMM3  | Reg::ZMM3  | 
            Reg::YMM4  | Reg::ZMM4  |
            Reg::YMM5  | Reg::ZMM5  |
            Reg::YMM6  | Reg::ZMM6  |
            Reg::YMM7  | Reg::ZMM7  |
            Reg::YMM8  | Reg::ZMM8  |
            Reg::XMM9  | Reg::YMM9  |  Reg::ZMM9  |
            Reg::XMM10 | Reg::YMM10 |  Reg::ZMM10 |
            Reg::XMM11 | Reg::YMM11 |  Reg::ZMM11 |
            Reg::XMM12 | Reg::YMM12 |  Reg::ZMM12 |
            Reg::XMM13 | Reg::YMM13 |  Reg::ZMM13 |
            Reg::XMM14 | Reg::YMM14 |  Reg::ZMM14 |
            Reg::XMM15 | Reg::YMM15 |  Reg::ZMM15 |
            Reg::XMM16 | Reg::YMM16 |  Reg::ZMM16 |
            Reg::XMM17 | Reg::YMM17 |  Reg::ZMM17 |
            Reg::XMM18 | Reg::YMM18 |  Reg::ZMM18 |
            Reg::XMM19 | Reg::YMM19 |  Reg::ZMM19 |
            Reg::XMM20 | Reg::YMM20 |  Reg::ZMM20 |
            Reg::XMM21 | Reg::YMM21 |  Reg::ZMM21 |
            Reg::XMM22 | Reg::YMM22 |  Reg::ZMM22 |
            Reg::XMM23 | Reg::YMM23 |  Reg::ZMM23 |
            Reg::XMM24 | Reg::YMM24 |  Reg::ZMM24 |
            Reg::XMM25 | Reg::YMM25 |  Reg::ZMM25 |
            Reg::XMM26 | Reg::YMM26 |  Reg::ZMM26 |
            Reg::XMM27 | Reg::YMM27 |  Reg::ZMM27 |
            Reg::XMM28 | Reg::YMM28 |  Reg::ZMM28 |
            Reg::XMM29 | Reg::YMM29 |  Reg::ZMM29 |
            Reg::XMM30 | Reg::YMM30 |  Reg::ZMM30 |
            Reg::XMM31 | Reg::YMM31 |  Reg::ZMM31 => true,
            _ => false
       }
    }
    
    pub fn is_general(&self) -> bool {
        match *self {
            Reg::AL | Reg::AH | Reg::AX | Reg::EAX | Reg::RAX |
            Reg::BL | Reg::BH | Reg::BX | Reg::EBX | Reg::RBX |
            Reg::CL | Reg::CH | Reg::CX | Reg::ECX | Reg::RCX |
            Reg::DL | Reg::DH | Reg::DX | Reg::EDX | Reg::RDX |
            Reg::SI | Reg::ESI | Reg::RSI |
            Reg::DI | Reg::EDI | Reg::RDI |
            Reg::SP | Reg::ESP | Reg::RSP |
            Reg::BP | Reg::EBP | Reg::RBP |
            Reg::R8  | Reg::R8D  | Reg::R8W  | Reg::R8B  |
            Reg::R9  | Reg::R9D  | Reg::R9W  | Reg::R9B  |
            Reg::R10 | Reg::R10D | Reg::R10W | Reg::R10B |
            Reg::R11 | Reg::R11D | Reg::R11W | Reg::R11B |
            Reg::R12 | Reg::R12D | Reg::R12W | Reg::R12B |
            Reg::R13 | Reg::R13D | Reg::R13W | Reg::R13B |
            Reg::R14 | Reg::R14D | Reg::R14W | Reg::R14B |
            Reg::R15 | Reg::R15D | Reg::R15W | Reg::R15B => true,
            _ => false
        }
    }

    pub fn is_fpu(&self) -> bool {
        match *self {
            Reg::ST0 | Reg::ST1 | Reg::ST2 | Reg::ST3 |
            Reg::ST4 | Reg::ST5 | Reg::ST6 | Reg::ST7 |
            Reg::ST => true,
            _=> false
        }
    }

    pub fn is_mmx(&self) -> bool {
        match *self {
            Reg::MM0 | Reg::MM1 | Reg::MM2 | Reg::MM3 | 
            Reg::MM4 | Reg::MM5 | Reg::MM6 | Reg::MM7 => true,
            _=> false
        }
    }

    pub fn is_sse(&self) -> bool {
        match *self {
            Reg::XMM0  | Reg::XMM1  | Reg::XMM2  | Reg::XMM3  | 
            Reg::XMM4  | Reg::XMM5  | Reg::XMM6  | Reg::XMM7  | 
            Reg::XMM8  | Reg::XMM9  | Reg::XMM10 | Reg::XMM11 | 
            Reg::XMM12 | Reg::XMM13 | Reg::XMM14 | Reg::XMM15 => true,
            _ => false
        }
    }

    pub fn is_avx(&self) -> bool {
        match *self {
            Reg::XMM0  | Reg::YMM0  | Reg::ZMM0   | 
            Reg::XMM1  | Reg::YMM1  | Reg::ZMM1   |
            Reg::XMM2  | Reg::YMM2  | Reg::ZMM2   |
            Reg::XMM3  | Reg::YMM3  | Reg::ZMM3   | 
            Reg::XMM4  | Reg::YMM4  | Reg::ZMM4   |
            Reg::XMM5  | Reg::YMM5  | Reg::ZMM5   |
            Reg::XMM6  | Reg::YMM6  | Reg::ZMM6   |
            Reg::XMM7  | Reg::YMM7  | Reg::ZMM7   |
            Reg::XMM8  | Reg::YMM8  | Reg::ZMM8   |
            Reg::XMM9  | Reg::YMM9  |  Reg::ZMM9  |
            Reg::XMM10 | Reg::YMM10 |  Reg::ZMM10 |
            Reg::XMM11 | Reg::YMM11 |  Reg::ZMM11 |
            Reg::XMM12 | Reg::YMM12 |  Reg::ZMM12 |
            Reg::XMM13 | Reg::YMM13 |  Reg::ZMM13 |
            Reg::XMM14 | Reg::YMM14 |  Reg::ZMM14 |
            Reg::XMM15 | Reg::YMM15 |  Reg::ZMM15 |
            Reg::XMM16 | Reg::YMM16 |  Reg::ZMM16 |
            Reg::XMM17 | Reg::YMM17 |  Reg::ZMM17 |
            Reg::XMM18 | Reg::YMM18 |  Reg::ZMM18 |
            Reg::XMM19 | Reg::YMM19 |  Reg::ZMM19 |
            Reg::XMM20 | Reg::YMM20 |  Reg::ZMM20 |
            Reg::XMM21 | Reg::YMM21 |  Reg::ZMM21 |
            Reg::XMM22 | Reg::YMM22 |  Reg::ZMM22 |
            Reg::XMM23 | Reg::YMM23 |  Reg::ZMM23 |
            Reg::XMM24 | Reg::YMM24 |  Reg::ZMM24 |
            Reg::XMM25 | Reg::YMM25 |  Reg::ZMM25 |
            Reg::XMM26 | Reg::YMM26 |  Reg::ZMM26 |
            Reg::XMM27 | Reg::YMM27 |  Reg::ZMM27 |
            Reg::XMM28 | Reg::YMM28 |  Reg::ZMM28 |
            Reg::XMM29 | Reg::YMM29 |  Reg::ZMM29 |
            Reg::XMM30 | Reg::YMM30 |  Reg::ZMM30 |
            Reg::XMM31 | Reg::YMM31 |  Reg::ZMM31 => true,
            _ => false
        }
    }

    pub fn size(&self) -> OperandSize {
        match *self {
            // 8-bit registers
            Reg::AL   | Reg::AH   | Reg::BL   | Reg::BH   |        
            Reg::CL   | Reg::CH   | Reg::DL   | Reg::DH   |        
            Reg::R8B  | Reg::R9B  | Reg::R10B | Reg::R11B | 
            Reg::R12B | Reg::R13B | Reg::R14B | Reg::R15B => OperandSize::Byte,
            
            // 16-bit regiters
            Reg::AX   | Reg::BX   | Reg::CX   | Reg::DX   |        
            Reg::BP   | Reg::SP   | Reg::SI   | Reg::DI   |
            Reg::R8W  | Reg::R9W  | Reg::R10W | Reg::R11W | 
            Reg::R12W | Reg::R13W | Reg::R14W | Reg::R15W |
            Reg::FLAGS | Reg::IP => OperandSize::Word,

            // 32-bit registers
            Reg::EAX  | Reg::EBX  | Reg::ECX  | Reg::EDX  |        
            Reg::EBP  | Reg::ESP  | Reg::ESI  | Reg::EDI  |
            Reg::R8D  | Reg::R9D  | Reg::R10D | Reg::R11D | 
            Reg::R12D | Reg::R13D | Reg::R14D | Reg::R15D |
            Reg::CR0  | Reg::CR1  | Reg::CR2  | Reg::CR3  |
            Reg::DR0 | Reg::DR1 | Reg::DR2 | Reg::DR3 | 
            Reg::DR4 | Reg::DR5 | Reg::DR6 | Reg::DR7 |
            Reg::TR3  | Reg::TR4  | Reg::TR5  | Reg::TR6  | // TODO Are test registers really 32 bits?
            Reg::TR7  | Reg::CR4  | Reg::EFLAGS | Reg::EIP  => OperandSize::Dword,

            // 64-bit registers
            Reg::RAX  | Reg::RBX  | Reg::RCX  | Reg::RDX  |        
            Reg::RBP  | Reg::RSP  | Reg::RSI  | Reg::RDI  |
            Reg::R8   | Reg::R9   | Reg::R10  | Reg::R11  | 
            Reg::R12  | Reg::R13  | Reg::R14  | Reg::R15  |
            Reg::MM0  | Reg::MM1  | Reg::MM2  | Reg::MM3  |
            Reg::MM4  | Reg::MM5  | Reg::MM6  | Reg::MM7  |
            Reg::CR8  | Reg::RFLAGS | Reg::RIP => OperandSize::Qword,

            // 80-bit registers
            Reg::ST  | Reg::ST0 | Reg::ST1 | Reg::ST2 | Reg::ST3 | 
            Reg::ST4 | Reg::ST5 | Reg::ST6 | Reg::ST7 => OperandSize::Tbyte,

            // 128-bit registers
            Reg::XMM0  | Reg::XMM1  | Reg::XMM2  | Reg::XMM3  |
            Reg::XMM4  | Reg::XMM5  | Reg::XMM6  | Reg::XMM7  |
            Reg::XMM8  | Reg::XMM9  | Reg::XMM10 | Reg::XMM11 |
            Reg::XMM12 | Reg::XMM13 | Reg::XMM14 | Reg::XMM15 |
            Reg::XMM16 | Reg::XMM17 | Reg::XMM18 | Reg::XMM19 |
            Reg::XMM20 | Reg::XMM21 | Reg::XMM22 | Reg::XMM23 |
            Reg::XMM24 | Reg::XMM25 | Reg::XMM26 | Reg::XMM27 |
            Reg::XMM28 | Reg::XMM29 | Reg::XMM30 | Reg::XMM31 |
            Reg::BND0  | Reg::BND1  | Reg::BND2  | Reg::BND3  => OperandSize::XMMword,

            // 256-bit registers
            Reg::YMM0  | Reg::YMM1  | Reg::YMM2  | Reg::YMM3  |
            Reg::YMM4  | Reg::YMM5  | Reg::YMM6  | Reg::YMM7  |
            Reg::YMM8  | Reg::YMM9  | Reg::YMM10 | Reg::YMM11 |
            Reg::YMM12 | Reg::YMM13 | Reg::YMM14 | Reg::YMM15 |
            Reg::YMM16 | Reg::YMM17 | Reg::YMM18 | Reg::YMM19 |
            Reg::YMM20 | Reg::YMM21 | Reg::YMM22 | Reg::YMM23 |
            Reg::YMM24 | Reg::YMM25 | Reg::YMM26 | Reg::YMM27 |
            Reg::YMM28 | Reg::YMM29 | Reg::YMM30 | Reg::YMM31 => OperandSize::YMMword,

            // 512-bit registers
            Reg::ZMM0  | Reg::ZMM1  | Reg::ZMM2  | Reg::ZMM3  |
            Reg::ZMM4  | Reg::ZMM5  | Reg::ZMM6  | Reg::ZMM7  |
            Reg::ZMM8  | Reg::ZMM9  | Reg::ZMM10 | Reg::ZMM11 |
            Reg::ZMM12 | Reg::ZMM13 | Reg::ZMM14 | Reg::ZMM15 |
            Reg::ZMM16 | Reg::ZMM17 | Reg::ZMM18 | Reg::ZMM19 |
            Reg::ZMM20 | Reg::ZMM21 | Reg::ZMM22 | Reg::ZMM23 |
            Reg::ZMM24 | Reg::ZMM25 | Reg::ZMM26 | Reg::ZMM27 |
            Reg::ZMM28 | Reg::ZMM29 | Reg::ZMM30 | Reg::ZMM31 => OperandSize::ZMMword,

            // Mask registers
            Reg::K0 | Reg::K1 | Reg::K2 | Reg::K3 |
            Reg::K4 | Reg::K5 | Reg::K6 | Reg::K7 => OperandSize::Word, // TODO Is this size correct?

            // TODO
            // Mode dependent (16/32/64)
            Reg::CS  | Reg::DS  | Reg::ES  | Reg::FS  |
            Reg::GS  | Reg::SS => OperandSize::Word, 

            // TODO?
            Reg::GDTR | Reg::LDTR | Reg::IDTR | Reg::TR |
            Reg::XCR  | Reg::MSR  | Reg::PMC => OperandSize::Dword
        }
    }

    pub fn is_8_bit(&self) -> bool { self.size() == OperandSize::Byte }
    pub fn is_16_bit(&self) -> bool { self.size() == OperandSize::Word }
    pub fn is_32_bit(&self) -> bool { self.size() == OperandSize::Dword }
    pub fn is_64_bit(&self) -> bool { self.size() == OperandSize::Qword }
    pub fn is_128_bit(&self) -> bool { self.size() == OperandSize::XMMword }
    pub fn is_256_bit(&self) -> bool { self.size() == OperandSize::YMMword }
    pub fn is_512_bit(&self) -> bool { self.size() == OperandSize::ZMMword }

    pub fn is_control(&self) -> bool {
        match *self {
            Reg::CR0 | Reg::CR1 | Reg::CR2 | Reg::CR3 |
            Reg::CR4 | Reg::CR8 => true,
            _ => false
        }
    }

    pub fn is_debug(&self) -> bool {
        match *self {
            Reg::DR0 | Reg::DR1 | Reg::DR2 | Reg::DR3 | 
            Reg::DR4 | Reg::DR5 | Reg::DR6 | Reg::DR7 => true,
            _ => false
        }
    }

    pub fn is_flags(&self) -> bool {
        match *self {
            Reg::FLAGS | Reg::EFLAGS | Reg::RFLAGS => true,
            _ => false
        }
    }

    pub fn is_segment(&self) -> bool {
        match *self {
            Reg::CS | Reg::DS | Reg::ES |
            Reg::FS | Reg::GS | Reg::SS => true,
            _ => false
        }
    }

    pub fn is_test(&self) -> bool {
        match *self {
            Reg::TR3 | Reg::TR4 | Reg::TR5 |
            Reg::TR6 | Reg::TR7 => true,
            _ => false
        }
    }

    pub fn is_mask(&self) -> bool {
        match *self {
            Reg::K0 | Reg::K1 | Reg::K2 | Reg::K3 |
            Reg::K4 | Reg::K5 | Reg::K6 | Reg::K7 => true,
            _ => false
        }
    }

    pub fn is_bounds(&self) -> bool {
        match *self {
            Reg::BND0 | Reg::BND1 | Reg::BND2 | Reg::BND3 => true,
            _ => false
        }
    }

    pub fn get_reg_code(&self) -> u8 {
        match *self {
            // TODO Handle SPL, BPL, SIL, DIL, AVX16-3j
            Reg::AL   | Reg::AX   | Reg::EAX  | Reg::RAX | Reg::ST0 | Reg::MM0 | Reg::XMM0  | Reg::YMM0  | Reg::ZMM0  | Reg::ES | Reg::CR0             | Reg::K0 | Reg::BND0 | Reg::DR0 => 0,
            Reg::CL   | Reg::CX   | Reg::ECX  | Reg::RCX | Reg::ST1 | Reg::MM1 | Reg::XMM1  | Reg::YMM1  | Reg::ZMM1  | Reg::CS | Reg::CR1             | Reg::K1 | Reg::BND1 | Reg::DR1 => 1,
            Reg::DL   | Reg::DX   | Reg::EDX  | Reg::RDX | Reg::ST2 | Reg::MM2 | Reg::XMM2  | Reg::YMM2  | Reg::ZMM2  | Reg::SS | Reg::CR2             | Reg::K2 | Reg::BND2 | Reg::DR2 => 2,
            Reg::BL   | Reg::BX   | Reg::EBX  | Reg::RBX | Reg::ST3 | Reg::MM3 | Reg::XMM3  | Reg::YMM3  | Reg::ZMM3  | Reg::DS | Reg::CR3  | Reg::DR3 | Reg::K3 | Reg::BND3 | Reg::DR3 => 3,
            Reg::AH   | Reg::SP   | Reg::ESP  | Reg::RSP | Reg::ST4 | Reg::MM4 | Reg::XMM4  | Reg::YMM4  | Reg::ZMM4  | Reg::FS | Reg::CR4  | Reg::DR4 | Reg::K4             | Reg::DR4 => 4,
            Reg::CH   | Reg::BP   | Reg::EBP  | Reg::RBP | Reg::ST5 | Reg::MM5 | Reg::XMM5  | Reg::YMM5  | Reg::ZMM5  | Reg::GS             | Reg::DR5 | Reg::K5             | Reg::DR5 => 5,
            Reg::DH   | Reg::SI   | Reg::ESI  | Reg::RSI | Reg::ST6 | Reg::MM6 | Reg::XMM6  | Reg::YMM6  | Reg::ZMM6                        | Reg::DR6 | Reg::K6             | Reg::DR6 => 6,
            Reg::BH   | Reg::DI   | Reg::EDI  | Reg::RDI | Reg::ST7 | Reg::MM7 | Reg::XMM7  | Reg::YMM7  | Reg::ZMM7                        | Reg::DR7 | Reg::K7             | Reg::DR7 => 7,
            Reg::R8B  | Reg::R8W  | Reg::R8D  | Reg::R8                        | Reg::XMM8  | Reg::YMM8  | Reg::ZMM8  | Reg::ES | Reg::CR8                                              => 8,
            Reg::R9B  | Reg::R9W  | Reg::R9D  | Reg::R9                        | Reg::XMM9  | Reg::YMM9  | Reg::ZMM9  | Reg::CS                                                         => 9,
            Reg::R10B | Reg::R10W | Reg::R10D | Reg::R10                       | Reg::XMM10 | Reg::YMM10 | Reg::ZMM10 | Reg::SS                                                         => 10,
            Reg::R11B | Reg::R11W | Reg::R11D | Reg::R11                       | Reg::XMM11 | Reg::YMM11 | Reg::ZMM11 | Reg::DS                                                         => 11,
            Reg::R12B | Reg::R12W | Reg::R12D | Reg::R12                       | Reg::XMM12 | Reg::YMM12 | Reg::ZMM12 | Reg::FS                                                         => 12,
            Reg::R13B | Reg::R13W | Reg::R13D | Reg::R13                       | Reg::XMM13 | Reg::YMM13 | Reg::ZMM13 | Reg::GS                                                         => 13,
            Reg::R14B | Reg::R14W | Reg::R14D | Reg::R14                       | Reg::XMM14 | Reg::YMM14 | Reg::ZMM14                                                                   => 14,
            Reg::R15B | Reg::R15W | Reg::R15D | Reg::R15                       | Reg::XMM15 | Reg::YMM15 | Reg::ZMM15                                                                   => 15,
            _ => panic!("Invalid register.")
        }
    }
}

// AVX Merge Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MergeMode {
    Merge,
    Zero
}

// AVX Mask Register (k0-k7)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MaskReg {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7
}

impl MaskReg {
    pub fn get_reg_code(&self) -> u8 {
        match *self {
            MaskReg::K0 => 0,
            MaskReg::K1 => 1,
            MaskReg::K2 => 2,
            MaskReg::K3 => 3,
            MaskReg::K4 => 4,
            MaskReg::K5 => 5,
            MaskReg::K6 => 6,
            MaskReg::K7 => 7,
        }
    }
}

// AVX Broadcast Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BroadcastMode {
    Broadcast1To2,
    Broadcast1To4,
    Broadcast1To8,
    Broadcast1To16,
}

impl BroadcastMode {
    pub fn get_multiplier(&self) -> u32 {
        match *self {
            BroadcastMode::Broadcast1To2 => 2,
            BroadcastMode::Broadcast1To4 => 4,
            BroadcastMode::Broadcast1To8 => 8,
            BroadcastMode::Broadcast1To16 => 16,
        }
    }
}
