use instruction::Reg;
use operand::OperandSize;

pub struct InstructionDefinition {
    pub mnemonic: String,
    pub allow_prefix: bool,
    pub operand_size_prefix: OperandSizePrefixBehavior,
    pub address_size_prefix: Option<bool>,
    pub f2_prefix: PrefixBehavior,
    pub f3_prefix: PrefixBehavior,
    pub composite_prefix: Option<CompositePrefix>,
    pub fwait: bool,

    pub two_byte_opcode: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub opcode_ext: Option<u8>,

    pub has_mod_rm: bool,
    pub fixed_mod_rm_mod: Option<u8>,
    pub fixed_mod_rm_reg: Option<u8>,
    pub allow_mask: bool,
    pub allow_merge_mode: bool,
    pub allow_rounding: bool,
    pub allow_sae: bool,
    
    pub operands: [Option<OperandDefinition>; 4],

    pub feature_set: Option<&'static [FeatureSet]>,
    pub valid_16: bool,
    pub valid_32: bool,
    pub valid_64: bool,
    pub desc: &'static str,
}

impl InstructionDefinition {
    pub fn add_operand(&mut self, operand: OperandDefinition) {
        if self.operands[0].is_none() { self.operands[0] = Some(operand); }
        else if self.operands[1].is_none() { self.operands[1] = Some(operand); }
        else if self.operands[2].is_none() { self.operands[2] = Some(operand); }
        else if self.operands[3].is_none() { self.operands[3] = Some(operand); }
        else { panic!("Operands full"); }
    }
}

impl Default for InstructionDefinition {
    fn default() -> InstructionDefinition {
        InstructionDefinition {
            mnemonic: String::from(""),
            allow_prefix: true,
            operand_size_prefix: OperandSizePrefixBehavior::Never,
            address_size_prefix: None,
            composite_prefix: None,
            fwait: false,
            f2_prefix: PrefixBehavior::Never,
            f3_prefix: PrefixBehavior::Never,

            two_byte_opcode: false,
            primary_opcode: 0,
            secondary_opcode: None,
            opcode_ext: None,

            has_mod_rm: false,
            fixed_mod_rm_mod: None,
            fixed_mod_rm_reg: None,
            allow_mask: false,
            allow_merge_mode: false,
            allow_rounding: false,
            allow_sae: false,

            operands: [None, None, None, None],

            feature_set: None,
            valid_16: false,
            valid_32: false,
            valid_64: false,
            desc: ""
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperandSizePrefixBehavior {
    Always,
    RealOnly,
    NotReal,
    Never
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrefixBehavior {
    Always,
    Optional,
    Never
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositePrefix {
    Rex {
        size_64: Option<bool>,
    },

    Vex {
        vector_size: Option<OperandSize>,
        operand_behavior: Option<VexOperandBehavior>,
        we: Option<bool>
    },

    Evex {
        vector_size: Option<OperandSize>,
        operand_behavior: Option<VexOperandBehavior>,
        we: Option<bool>,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FeatureSet {
    Sse,
    Sse2,
    Sse3,
    Sse41,
    Sse42,
    Aesni,
    Pclmulqdq,
    Avx,
    Avx512vl,
    Avx512f,
    Rdrand
}

#[derive(Clone, Debug)]
pub struct OperandDefinition {
    pub encoding: OperandEncoding,
    pub access: OperandAccess,
    pub size: OperandSize,
    pub op_type: OperandType
}

#[derive(Clone, Debug)]
pub enum OperandType {
    Reg(RegType),
    Mem(Option<OperandSize>),
    Imm,
    Constant,
    Offset,
    Rel(OperandSize),
    Mib,
    Bcst(OperandSize),
    Fixed(FixedOperand),
    Set(Vec<OperandType>)
}

impl OperandType {
    pub fn is_mem(&self) -> bool {
        match *self {
            OperandType::Mem(_) |
            OperandType::Rel(_) |
            OperandType::Offset |
            OperandType::Bcst(_) => true,
            _ => false
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OperandEncoding {
    ModRmReg,
    ModRmRm,
    Vex,
    Imm,
    OpcodeAddend,
    Offset,
    Mib,
    Fixed
}

#[derive(Clone, Copy, Debug)]
pub enum OperandAccess {
    Read,
    Write,
    ReadWrite
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegType {
    General,
    Mmx,
    Avx,
    Fpu,
    Bound,
    Mask,
    Segment,
    Control,
    Debug
}

#[derive(Clone, Copy, Debug)]
pub enum FixedOperand {
    Reg(Reg),
    Constant(u32)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VexOperandBehavior {
    Nds,
    Ndd,
    Dds
}
