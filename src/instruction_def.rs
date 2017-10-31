use std::collections::HashMap;
use std::io::Write;
use std::sync::RwLock;
use ::{Instruction, InstructionEncodingError, Mnemonic, Mode, Operand, OperandSize, Reg, RegType};
use ::instruction_buffer::InstructionBuffer;
use ::instruction_defs::INSTR_DEFS;

#[derive(Debug)]
pub struct InstructionDefinition {
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
    pub mnemonic: Mnemonic,
}

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
        .ok_or(InstructionEncodingError::NoEncoding)
        .and_then(|list| {
            let matches = list.iter().filter(|enc| enc.matches_instruction(instr, mode));
            let mut sizes = None;
            let mut best = None;
            
            // Look for the shortest matching def. If multiple incompatible defs match,
            // return AmbiguousSize.
            for m in matches {
                if best.map_or(true, |b| compare_def_length(m, b)) {
                    best = Some(m);
                }
                
                let m_sizes = get_op_sizes(m, instr);
                if sizes.map_or(true, |s| compare_sizes(&s, &m_sizes)) {
                    sizes = Some(m_sizes);
                } else {
                    return Err(InstructionEncodingError::AmbiguousSize);
                }
            }

            best.ok_or(InstructionEncodingError::NoEncoding)
        })
}

fn temp_helper(tag: &'static str, val: bool) -> bool {
    // println!(" - {}: {:?}", tag, val);
    val
}

pub fn find_instruction_def_by_opcode(buffer: &InstructionBuffer, mode: Mode)
    -> Result<&'static InstructionDefinition, FindInstructionDefByOpcodeError> {
    let mut matches = INSTR_DEFS.iter().filter(|def| {
        let has_opcode_addend = def.operands.iter().any(|o| o.as_ref().map_or(false,
            |op| op.encoding == OperandEncoding::OpcodeAddend));

        def.two_byte_opcode == buffer.is_two_byte_opcode &&
        (def.primary_opcode == buffer.primary_opcode ||
        has_opcode_addend && (buffer.primary_opcode & !0x7) == def.primary_opcode) &&
        def.secondary_opcode == buffer.secondary_opcode &&
        match def.f2_prefix {
            PrefixBehavior::Always => buffer.f2_prefix,
            PrefixBehavior::Never => !buffer.f2_prefix,
            PrefixBehavior::Optional => true
        } &&
        match def.f3_prefix {
            PrefixBehavior::Always => buffer.f3_prefix,
            PrefixBehavior::Never => !buffer.f3_prefix,
            PrefixBehavior::Optional => true
        } &&
        def.fixed_mod_rm_mod.map_or(true, |m| buffer.mod_rm_mod == Some(m)) &&
        def.fixed_mod_rm_reg.map_or(true, |m| buffer.mod_rm_reg == Some(m)) &&
        // Only compare extensions if one is provided. If an opcode extension is needed to
        // disambiguate, it will be checked below.
        temp_helper("opcode_ext", (def.opcode_ext.is_none() || 
            buffer.mod_rm_reg.map_or(true, |_| buffer.mod_rm_reg == def.opcode_ext))) &&
        temp_helper("mode", match mode {
            Mode::Real => def.valid_16,
            Mode::Protected => def.valid_32,
            Mode::Long => def.valid_64,
        }) && 
        temp_helper("op_size_prefix", match def.operand_size_prefix {
            OperandSizePrefixBehavior::Always => buffer.operand_size_prefix,
            OperandSizePrefixBehavior::RealOnly => if mode == Mode::Real 
                { buffer.operand_size_prefix } else { !buffer.operand_size_prefix },
            OperandSizePrefixBehavior::NotReal => if mode == Mode::Real
            { !buffer.operand_size_prefix } else { buffer.operand_size_prefix },
            OperandSizePrefixBehavior::Never => !buffer.operand_size_prefix
        }) &&
        temp_helper("composite_prefix", match def.composite_prefix {
            Some(CompositePrefix::Rex { size_64: s }) => 
                buffer.composite_prefix == Some(::instruction_buffer::CompositePrefix::Rex) &&
                s.map_or(true, |size| size == buffer.operand_size_64),
            Some(CompositePrefix::Vex { vector_size: vs, we: we, .. }) => 
                buffer.composite_prefix == Some(::instruction_buffer::CompositePrefix::Vex) &&
                temp_helper("we", we.map_or(true, |e| buffer.vex_e == Some(e))) &&
                temp_helper("size", match vs {
                    Some(OperandSize::Xmmword) =>
                        buffer.vector_len == Some(false) && buffer.vex_l.map_or(true, |l| !l),
                    Some(OperandSize::Ymmword) =>
                        buffer.vector_len == Some(true) && buffer.vex_l.map_or(true, |l| !l),
                    Some(OperandSize::Zmmword) =>
                        buffer.vector_len == Some(false) && buffer.vex_l == Some(true),
                    _ => true
                }),
            Some(CompositePrefix::Evex { vector_size: vs, we: we, .. }) => 
                buffer.composite_prefix == Some(::instruction_buffer::CompositePrefix::Evex) &&
                we.map_or(true, |e| buffer.vex_e == Some(e)) &&
                match vs {
                    Some(OperandSize::Xmmword) =>
                        buffer.vector_len == Some(false) && buffer.vex_l.map_or(true, |l| !l),
                    Some(OperandSize::Ymmword) =>
                        buffer.vector_len == Some(true) && buffer.vex_l.map_or(true, |l| !l),
                    Some(OperandSize::Zmmword) =>
                        buffer.vector_len == Some(false) && buffer.vex_l == Some(true),
                    _ => true
                },
            None => buffer.composite_prefix != Some(::instruction_buffer::CompositePrefix::Vex)
                && buffer.composite_prefix != Some(::instruction_buffer::CompositePrefix::Evex)
                && !buffer.operand_size_64 // TODO Is this okay?
        }) &&
        temp_helper("operands", def.operands.iter().all(|op| op.as_ref().map_or(true, |o| o.is_compatible(buffer))))
    });

    let first = matches.next();
    if let Some(f) = first {
        let next = matches.next();
        if next.is_some() { // If multiple matches exist...
            if buffer.mod_rm_mod.is_none() { // Need a ModR/M byte
                Err(FindInstructionDefByOpcodeError::NeedModRm)
            } else {
                Ok(f)
                // TODO Is this an error case?
                // panic!("Internal error - multiple matching instruction definitions: \n\n{:?}\n\n{:?}", f, next.unwrap());
            }
        } else { // If there's only one match, it's the one we're looking for
            Ok(f)
        }
    } else {
        Err(FindInstructionDefByOpcodeError::NoEncoding)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindInstructionDefByOpcodeError {
    // Indicates that no matching definition exists
    NoEncoding,

    // Indicates that an opcode extension is needed to disambiguate
    NeedModRm
}

fn compare_sizes(a: &[Option<OperandSize>; 4], b: &[Option<OperandSize>; 4]) -> bool {
    a.iter().zip(b.iter()).all(|(m_s1, m_s2)| match (*m_s1, *m_s2) {
        (Some(s1), Some(s2)) => s1 == s2 || s1 == OperandSize::Unsized || s2 == OperandSize::Unsized,
        (None, None) => true,
        _ => false
    })
}

fn get_op_sizes(def: &InstructionDefinition, instr: &Instruction) -> [Option<OperandSize>; 4] {
    let mut ops = instr.operands();
    let mut iter = def.operands.iter().zip(ops.iter())
        .map(|(def, op)| def.as_ref().map(|d| op.map_or(d.size, |o| d.get_real_size(&o))));
    [iter.next().unwrap_or(None),
     iter.next().unwrap_or(None),
     iter.next().unwrap_or(None),
     iter.next().unwrap_or(None)]
}

fn compare_def_length(a: &InstructionDefinition, b: &InstructionDefinition) -> bool {
    a.len() < b.len()
}

impl InstructionDefinition {
    fn matches_instruction(&self, instr: &Instruction, mode: Mode) -> bool {
        self.mnemonic == instr.mnemonic &&
        // (self.allow_lock || instr.lock) &&
        (self.allow_rounding || instr.rounding_mode.is_none()) &&
        (self.allow_sae || !instr.sae) &&
        (self.allow_mask || instr.mask.is_none()) &&
        (self.operands.iter().zip(instr.operands().iter()).all(
            |(def, op)| if let Some(ref d) = *def {
                d.matches_operand(op, self, instr)
            } else { op.is_none() }
        )) &&
        match mode {
            Mode::Real => self.valid_16,
            Mode::Protected => self.valid_32,
            Mode::Long => self.valid_64
        }
    }

    // This isn't intended to be an exact byte length of the instruction, as it's only used to
    // compare definitions to find the shortest.
    fn len(&self) -> u32 {
        fn b(v: bool) -> u32 { if v { 1 } else { 0 } }

        (match self.composite_prefix {
            Some(CompositePrefix::Evex { .. }) => 4,
            Some(CompositePrefix::Vex { .. }) => 3, // TODO This could be made to determine vex len
            Some(CompositePrefix::Rex { .. }) => 1,
            None => 0
        }) +
        b(self.two_byte_opcode) +
        b(self.secondary_opcode.is_some()) +
        self.operands.iter().filter_map(|o| o.as_ref().map(|op| op.len())).sum::<u32>()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperandSizePrefixBehavior {
    Always,
    RealOnly,
    NotReal,
    Never
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefixBehavior {
    Always,
    Optional,
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
    pub fn matches_operand(&self, op: &Option<Operand>, instr_def: &InstructionDefinition, instr: &Instruction) -> bool {
        op.map(|o| {
            OperandDefinition::matches_operand_type(&self.op_type, self.size, op, instr_def, instr) &&
                if let Operand::Direct(reg) = o { // TODO Cleanup
                    if reg.is_avx() {
                        let code = reg.get_reg_code();
                        if code >= 0x10 {
                            if let Some(CompositePrefix::Evex { .. }) = instr_def.composite_prefix { true }
                            else { false }
                        } else if code >= 0x8 { 
                            if let Some(CompositePrefix::Evex { .. }) = instr_def.composite_prefix { true }
                            else if let Some(CompositePrefix::Vex { .. }) = instr_def.composite_prefix { true }
                            else { false }
                        } else { true }
                    } else { true }
                } else { true }
        }).unwrap_or(false)
    }

    fn get_real_size(&self, op: &Operand) -> OperandSize {
        self.get_real_type(op).map_or(OperandSize::Unsized, |t| {
            let s = match *t {
                OperandType::Mem(Some(s)) |
                OperandType::Bcst(s) => s,
                _ => self.size
            };

            match s {
                OperandSize::Far16 => OperandSize::Dword,
                OperandSize::Far32 => OperandSize::Fword,
                OperandSize::Far64 => OperandSize::Tbyte,
                _ => s
            }
        })
    }

    fn get_real_type(&self, op: &Operand) -> Option<&OperandType> {
        if let OperandType::Set(set) = self.op_type {
            set.iter().find(|s| match **s {
                OperandType::Reg(_) => {
                    !op.is_memory()
                },
                OperandType::Mem(s) => {
                    op.is_memory() &&
                    op.size().map_or(true, |sz| s.map_or(true, |sz2|
                        sz == sz2 ||
                        sz == OperandSize::Unsized ||
                        sz2 == OperandSize::Unsized))
                },
                OperandType::Bcst(s) => {
                    op.is_memory() &&
                    op.size().map_or(true, |sz|
                        sz == s ||
                        sz == OperandSize::Unsized ||
                        s == OperandSize::Unsized)
                },
                _ => unimplemented!()
            })
        } else { Some(&self.op_type) }
    }

    fn matches_operand_type(op_type: &OperandType, def_size: OperandSize, op: &Option<Operand>,
        instr_def: &InstructionDefinition, instr: &Instruction) -> bool {
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
                size_helper(op_size, op) &&
                instr.broadcast.map_or(false,
                    |b_m| op_size.bits() * b_m.get_multiplier() == def_size.bits()),
            OperandType::Fixed(fixed_op) => op.map(|o| fixed_op.matches_operand(&o))
                .unwrap_or(true),
            OperandType::Set(types) => types.iter().any(
                |t| OperandDefinition::matches_operand_type(t, def_size, op, instr_def, instr))
        }
    }

    // This isn't intended to be an exact byte length of the instruction, as it's only used to
    // compare definitions to find the shortest.
    fn len(&self) -> u32 {
        if self.encoding == OperandEncoding::OpcodeAddend { return 0; }

        match self.op_type {
            OperandType::Reg(_) |
            OperandType::Mem(_) |
            OperandType::Bcst(_) |
            OperandType::Offset |
            OperandType::Mib |
            OperandType::Set(_)
                => 1,
            OperandType::Imm
                => self.size.bits() / 8,
            OperandType::Rel(s)
                => s.bits() / 8,
            OperandType::Fixed(_)
                => 0
        }
    }

    fn is_compatible(&self, buffer: &InstructionBuffer) -> bool {
        if let OperandEncoding::ModRmRm = self.encoding {
            if let OperandType::Reg(_) = self.op_type {
                buffer.mod_rm_mod.map_or(true, |m| m == 0b11)
            } else if let OperandType::Mem(_) = self.op_type {
                buffer.mod_rm_mod.map_or(true, |m| m != 0b11)
            } else { true }
        } else { true }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl OperandType {
    pub fn broadcast_size(&self) -> Option<OperandSize> {
        match *self {
            OperandType::Bcst(s) => Some(s),
            OperandType::Set(items) => items.iter().filter_map(|i| i.broadcast_size()).next(),
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
