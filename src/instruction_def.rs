use std::collections::HashMap;
use std::io::Write;
use std::sync::RwLock;
use ::{Instruction, InstructionEncodingError, Mnemonic, Mode, Operand, OperandSize, Reg, RegType};
use ::instruction_defs::INSTR_DEFS;

lazy_static! {
    static ref INSTR_MNEMONIC_MAP : RwLock<HashMap<Mnemonic, Vec<&'static InstructionDefinition>>> = {
        let mut lock = RwLock::new(HashMap::new());
        {
            let mut map = lock.write().unwrap();
            load_instructions(&mut map);
        }
        lock
    };
}

pub fn load_instructions(map: &mut HashMap<Mnemonic, Vec<&'static InstructionDefinition>>) {
    for instr in INSTR_DEFS.iter() {
        if !map.contains_key(&instr.mnemonic) {
            let new_list = Vec::new();
            map.insert(instr.mnemonic, new_list);
        };
        let mut list = map.get_mut(&instr.mnemonic).unwrap();
        list.push(&instr);
    }
}

pub fn find_instruction_def(instr: &Instruction, mode: Mode) 
    -> Result<&'static InstructionDefinition, InstructionEncodingError> {
    INSTR_MNEMONIC_MAP.read().unwrap().get(&instr.mnemonic)
        .and_then(|list| list.iter().find(|enc| enc.matches_instruction(instr)))
        .map(|&i| i)
        .ok_or(InstructionEncodingError::NoEncoding)
}

impl InstructionDefinition {
    fn matches_instruction(&self, instr: &Instruction) -> bool {
        self.mnemonic == instr.mnemonic &&
        // (self.allow_lock || instr.lock) &&
        (self.allow_rounding || instr.rounding_mode.is_none()) &&
        (self.allow_sae || !instr.sae) &&
        (self.allow_mask || instr.mask.is_none()) &&
        (self.operands.iter().zip(instr.operands().iter()).all(
            |(def, op)| if let Some(ref d) = *def {
                d.matches_operand(op, instr)
            } else { op.is_none() }
        ))
    }
}

#[derive(Debug)]
pub struct InstructionDefinition {
    pub allow_prefix: bool,
    pub operand_size_prefix: OperandSizePrefixBehavior,
    pub address_size_prefix: Option<bool>,
    pub fixed_prefix: Option<u8>,
    pub composite_prefix: Option<CompositePrefix>,

    pub two_byte_opcode: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub opcode_ext: Option<u8>,

    pub fixed_post: Option<u8>,
    pub has_mod_rm: bool,
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
    pub mnemonic: Mnemonic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperandSizePrefixBehavior {
    Always,
    ConditionalOnSize(u8),
    Never
}

#[derive(Clone, Copy, Debug)]
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

impl OperandDefinition {
    pub fn matches_operand(&self, op: &Option<Operand>, instr: &Instruction) -> bool {
        op.map(|o| {
            OperandDefinition::matches_operand_type(&self.op_type, self.size, op, instr)
        }).unwrap_or(false)
    }

    fn matches_operand_type(op_type: &OperandType, def_size: OperandSize, op: &Option<Operand>, instr: &Instruction)
        -> bool {
        fn size_helper(def_size: OperandSize, op: &Option<Operand>) -> bool {
            op.and_then(|o| o.size()).map(
                |s| s == def_size || s == OperandSize::Unsized ||
                def_size == OperandSize::Unsized).unwrap_or(true)
        }

        match *op_type {
            OperandType::Reg(reg_type)
                => if let Some(Operand::Direct(reg)) = *op { 
                    reg.get_reg_type() == reg_type && size_helper(def_size, op)
                } else { false },
            OperandType::Mem(s) => op.map(|o| o.is_memory()).unwrap_or(false) &&
                size_helper(s.unwrap_or(def_size), op),
            OperandType::Imm => op.map(|o| o.is_literal() || o.is_offset() || o.is_far())
                .unwrap_or(false) && size_helper(def_size, op),
            OperandType::Offset => unimplemented!(),
            OperandType::Rel(op_size) => match *op {
                Some(Operand::Offset(o, size, ..)) => op_size.is_valid_literal(o),
                Some(Operand::Literal8(_)) => true,
                Some(Operand::Literal16(_)) => op_size.bits() >= 16,
                Some(Operand::Literal32(_)) => op_size.bits() >= 32,
                Some(Operand::Literal64(_)) => op_size.bits() >= 64,
                _ => false
            },
            OperandType::Mib => op.map(|o| o.is_scaled_indexed()).unwrap_or(false), // TODO Size?
            OperandType::Bcst(op_size) => op.map(|o| o.is_memory()).unwrap_or(false) &&
                size_helper(op_size, op),
            OperandType::Fixed(fixed_op) => op.map(|o| fixed_op.matches_operand(&o))
                .unwrap_or(true),
            OperandType::Set(types) => types.iter().any(
                |t| OperandDefinition::matches_operand_type(t, def_size, op, instr))
        }
    }
}

#[derive(Clone, Debug)]
pub enum OperandType {
    Reg(RegType),
    Mem(Option<OperandSize>),
    Imm,
    Offset,
    Rel(OperandSize),
    Mib,
    Bcst(OperandSize),
    Fixed(FixedOperand),
    Set(&'static [OperandType])
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
    FixedPostAddend,
    Fixed
}

#[derive(Clone, Copy, Debug)]
pub enum OperandAccess {
    Read,
    Write,
    ReadWrite
}

#[derive(Clone, Copy, Debug)]
pub enum FixedOperand {
    Reg(Reg),
    Constant(u32)
}

impl FixedOperand {
    pub fn matches_operand(&self, op: &Operand) -> bool {
        match *self {
            FixedOperand::Reg(reg) => {
                if let Operand::Direct(r) = *op { reg == r } else { false }
            },
            FixedOperand::Constant(val) => {
                op.get_literal().map(|v| v as u32 == val).unwrap_or(false)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VexOperandBehavior {
    Nds,
    Ndd,
    Dds
}
