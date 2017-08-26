/*
 *  Instruction format and definitions from ref.x86asm.net
 */

use std::collections::HashMap;
use std::io::Write;
use std::sync::RwLock;
use ::{ Instruction, Operand, Mnemonic, Mode, Reg, OperandSize, RegScale, SegmentReg, ProcessorLevel, InstructionFlags, BroadcastMode, InstructionEncodingError };
use ::encoding::{encode};
use ::instruction_buffer::InstructionBuffer;
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

#[derive(Copy, Clone, Debug)]
enum InstructionMatch {
    Match(Option<OperandSize>),
    AmbiguousSize,
    NoMatch
}

pub fn find_instruction_def(instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<&InstructionDefinition, InstructionEncodingError> {
    if let Some(list) = INSTR_MNEMONIC_MAP.read().unwrap().get(&instr.mnemonic) {
        let encoding_matches = list.iter()
            .map(|enc| (enc, enc.matches_instruction(instr, mode, proc_level)) );
        let mut max: Option<(u32, _)> = None;
        let mut op_size = None;
        for enc_match in encoding_matches {
            match enc_match.1 {
                InstructionMatch::Match(size) => {
                    // If there are multiple possible matches, it's ambiguous
                    if let Some(s) = size {
                        if let Some(op_s) = op_size {
                            if op_s != s { return Err(InstructionEncodingError::AmbiguousSize); }
                        } else { op_size = size }
                    }
                    let specificity = enc_match.0.get_specificity();
                    if max.map(|m| specificity > m.0).unwrap_or(true) {
                        max = Some((specificity, enc_match.0))
                    }
                },
                InstructionMatch::AmbiguousSize => { return Err(InstructionEncodingError::AmbiguousSize); }
                InstructionMatch::NoMatch => {}
            }
        }
        match max {
            Some((_, enc)) => Ok(enc),
            None => Err(InstructionEncodingError::NoEncoding)
        }
    } else {
        Err(InstructionEncodingError::InvalidMnemonic)
    }
}

pub fn get_instruction_def_by_opcode(buffer: &InstructionBuffer, mode: Mode) -> Option<&'static InstructionDefinition> {
    INSTR_DEFS.iter().find(|def| def.primary_opcode == buffer.primary_opcode &&
                        def.secondary_opcode == buffer.secondary_opcode &&
                        def.is_two_byte_opcode == buffer.is_two_byte_opcode &&
                        (def.valid_in_long_mode || mode != Mode::Long) &&
                        match def.mode {
                            Mode::Real => true,
                            Mode::Protected => mode == Mode::Real || mode == Mode::Protected,
                            Mode::Long => mode == Mode::Long
                        } &&
                        def.fixed_prefix == buffer.fixed_prefix)
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

#[derive(Copy, Clone, Debug)]
pub struct InstructionDefinition {
    pub mnemonic: Mnemonic,
    pub fixed_prefix: Option<u8>,
    pub is_two_byte_opcode: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub flags: InstructionDefinitionFlags,
    pub opcode_ext: Option<u8>,
    pub has_destination: bool,
    pub proc_start: Option<ProcessorLevel>,
    pub proc_end: Option<ProcessorLevel>,
    pub mode: Mode,
    pub ring_level: RingLevel,
    pub can_lock: bool,
    pub valid_in_long_mode: bool,
    pub force_op_size_prefix: bool,
    pub force_addr_size_prefix: bool,
    pub op_size_64_behavior: OpSize64Behavior,
    pub force_vex: bool,
    pub force_evex: bool,
    pub vector_size: Option<bool>,
    pub allow_rounding_mode: bool,
    pub allow_sae: bool,
    pub allowed_broadcast: Option<BroadcastMode>,
    pub operand1: Option<OperandDescription>,
    pub operand2: Option<OperandDescription>,
    pub operand3: Option<OperandDescription>,
    pub operand4: Option<OperandDescription>,
}

impl InstructionDefinition {
    pub fn encode<W>(&self, writer: &mut W, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<usize, InstructionEncodingError>
        where W : Write {
        encode(writer, &self, instr, mode, proc_level)
    }

    pub fn matches_instruction(&self, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> InstructionMatch {
       let operand_pairs = [(&self.operand1, &instr.operand1),
        (&self.operand2, &instr.operand2),
        (&self.operand3, &instr.operand3),
        (&self.operand4, &instr.operand4)];

       // Check mode
       if !(match mode {
           Mode::Real => (self.mode == Mode::Real),
           Mode::Protected => (self.mode == Mode::Real || self.mode == Mode::Protected),
           Mode::Long => (self.mode == Mode::Real || self.mode == Mode::Protected || self.mode == Mode::Long) && self.valid_in_long_mode,
		} &&

       // Check flags
       (!instr.flags.lock || self.can_lock) &&

       // Check processor level
       self.proc_start.map(|p| proc_level >= p).unwrap_or(true) &&
       self.proc_end.map(|p| proc_level <= p).unwrap_or(true)

       ) { return InstructionMatch::NoMatch; }

        // Check operand size prefix. Each operand can require a specific prefix, no prefix, or
        // either, depending on oeprand size. To represent this, the operand pair iterator is
        // folded, with the accumulator being an Option<(bool, bool)>, where t.0 indicates whether no
        // operand prefix is valid, and t.1 represents whether the operand size prefix is present.
        // If none of the operands specifically require the operand size prefix to be or not to be
        // present, then accumulator will remain none, and no prefix will be emitted. If the
        // fold result is some and both values are true, then this operand is ambiguous, as the
        // prefix could be encoded or not encoded. If none are true, that indicates a mismatch. If
        // exactly one value is true, that determines whether or not the prefix is encoded.
    
        let op_size_pref = if let Some(size_pref_acc) = operand_pairs.iter().fold(None, |acc: Option<(bool, bool)>, pair| {
            pair.0.and_then(|op_def| {
                let maybe_pair_pref = op_def.get_operand_size_prefix(pair.1, mode);
                maybe_pair_pref.and_then(|pair_pref|
                    acc.map(|acc_i: (bool, bool)| (acc_i.0 & pair_pref.0, acc_i.1 & pair_pref.1) ).or(Some(pair_pref))
                )
            }).or(acc)
        }) {
           if size_pref_acc.0 & size_pref_acc.1 { return InstructionMatch::AmbiguousSize; }
           else if !size_pref_acc.0 & !size_pref_acc.1 { return InstructionMatch::NoMatch; }
           else { size_pref_acc.1 }
        } else { false };
        
        // Check REX.W
        let rex_w = (mode == Mode::Long) && if let Some(size_pref_acc) = operand_pairs.iter().fold(None, |acc: Option<(bool, bool)>, pair| {
            pair.0.and_then(|op_def| {
                let maybe_pair_pref = pair.1.as_ref().and_then(|o| op_def.get_rex_w(o));
                maybe_pair_pref.and_then(|pair_pref|
                    acc.map(|acc_i: (bool, bool)| (acc_i.0 & pair_pref.0, acc_i.1 & pair_pref.1) ).or(Some(pair_pref))
                )
            }).or(acc)
        }) {
           if size_pref_acc.0 & size_pref_acc.1 { return InstructionMatch::AmbiguousSize; }
           else if !size_pref_acc.0 & !size_pref_acc.1 { return InstructionMatch::NoMatch; }
           else { size_pref_acc.1 }
        } else { false };
        
        // Check operands and make sure size is unambiguous
        let mut op_matches = operand_pairs.iter()
            .map(|tuple| (tuple.0, tuple.1, match *tuple.0 {
                // If there's an expected operand, test the provided one against it
                Some(op_def) => Some(op_def.matches_operand(tuple.1, mode, 
                    // Some operations use sign extend to op1 to mean sign extend to other operand2
                    instr.operand1.as_ref().and_then(|d| d.size() ).or(instr.operand2.as_ref().and_then(|s| s.size() ) ))),
                // If there's no operand expected, but one was provided, it's an error
                None => tuple.1.as_ref().map(|_| false )
            }) );

        if !op_matches.filter_map(|tuple| tuple.2).all(|v| v) {
            return InstructionMatch::NoMatch;
        }

        // Check vector size
        let vector_size = match operand_pairs.iter()
            .filter_map(|tuple| tuple.0.and_then(
                |op_def| tuple.1.as_ref().and_then(|ref op|
                    // Only check vector size if there are multiple possibilities in order to
                    // prevent instructions with differing operand sizes from failing check
                    match op_def.operand_type {
                        OperandType::AVX |
                        OperandType::XMMorYMM |
                        OperandType::XMMorYMMorMemOrMem64 => op_def.get_vector_size(op).map(|v| (true, v)),
                        _ => op_def.get_vector_size(op).map(|v| (false, v))
                    } ) ) )
            .fold(Ok((false, None)), |acc, v| {
                acc.and_then(|acc_val| {
                    // v.0 represents whether the operand has multiple possible types (i.e.
                    // XMMorYMM). All operands with multiple possibilities need to match. If
                    // there aren't any operands where v.0 is true, then the vector size should
                    // be the size of the fixed operand sizes in order to handle the case of
                    // instruction encodings where there is only one possible vector size.
                    if v.0 && acc_val.0 {
                        if acc_val.1.map(|a| a != v.1).unwrap_or(false) { Err(InstructionEncodingError::MismatchedSize) }
                        else { Ok((true, Some(v.1))) }
                    } else {
                        if !acc_val.0 { Ok((v.0, Some(acc_val.1.map(|a: OperandSize| if v.1.bits() > a.bits() { v.1 } else { a }).unwrap_or(v.1)))) }
                        else { Ok(acc_val) }
                    }
                })}) {
                    Ok((_, v_size)) => v_size,
                    Err(InstructionEncodingError::MismatchedSize) => { return InstructionMatch::NoMatch }, // Vector size mismatch
                    _ => panic!("Internal error") // Shouldn't be possible
                };

        // Check broadcast semantics, if present
        if let Some((op, broadcast_mode)) = operand_pairs.iter()
               .map(|op_tuple| if let Some(ref op) = *op_tuple.1 {
                   match *op {
                       Operand::AVXBroadcastIndirect(b_mode, ..) |
                       Operand::AVXBroadcastIndirectDisplaced(b_mode, ..) |
                       Operand::AVXBroadcastIndirectScaledIndexed(b_mode, ..) |
                       Operand::AVXBroadcastIndirectScaledIndexedDisplaced(b_mode, ..)
                            => Some((op, b_mode)),
                       _ => None
                   } 
                } else { None } )
                .filter_map(|x| x).next() {

            if let Some(bcst) = self.allowed_broadcast {
                if bcst != broadcast_mode { return InstructionMatch::NoMatch; } // Disallowed broadcast mode
            } else if let Some(v_size) = vector_size {
                if let Some(o_size) = op.size() {
                    if !(v_size.bits() == o_size.bits() * match broadcast_mode {
                        BroadcastMode::Broadcast1To2 => 2,
                        BroadcastMode::Broadcast1To4 => 4,
                        BroadcastMode::Broadcast1To8 => 8,
                        BroadcastMode::Broadcast1To16 => 16
                    }) { return InstructionMatch::NoMatch; } // Broadcast mode mismatch
                }
            }
        }

        // Check avx semantics (sae/rounding mode)
        let has_memory_operands = operand_pairs.iter().any(|pair| pair.1.as_ref().map(|o| o.is_memory()).unwrap_or(false));
        let has_avx_operands = operand_pairs.iter().any(|pair| pair.0.map(|o| o.operand_type == OperandType::AVX).unwrap_or(false));
        if instr.flags.sae && !self.allow_sae { return InstructionMatch::NoMatch; } // SAE invalid
        if instr.flags.rounding_mode.is_some() && (!self.allow_rounding_mode || has_memory_operands ||
            (has_avx_operands && vector_size.map(|v| v != OperandSize::ZMMword).unwrap_or(false))) {
            return InstructionMatch::NoMatch; // Invalid rounding mode
        };

        let op_size = operand_pairs.iter().filter_map(|t| t.0.and_then(|def| def.get_size(op_size_pref, false, vector_size, mode ))).max_by_key(|s| s.bits() );

        // If we've made it this far, it's a match
        InstructionMatch::Match(op_size)
    }
    
    // Determines which encoding is used when multiple are possible. Higher value means
    // instruction is more specific (i.e. one byte int 3) and should be chosen. This
    // metric could likely be improved in the future.
    fn get_specificity(&self) -> u32 {
        // TODO Prefer 64-bit encodings (i.e. PUSH 50)
        fn test_op(op: &Option<OperandDescription>) -> u32 {
            if op.and_then(|op| op.fixed_operand).is_some() { 1 } else { 0 }
        }

        *(&[self.operand2, self.operand3, self.operand4, self.operand1].iter()
            .fold(0, |acc, op| acc + test_op(op)))
    }

    pub fn ordered_operands(&self) -> [&Option<OperandDescription>; 4] {
        if self.has_destination {
            [&self.operand2, &self.operand3, &self.operand4, &self.operand1]
        } else {
            [&self.operand1, &self.operand2, &self.operand3, &self.operand4]
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RingLevel {
    Ring0,
    Ring1,
    Ring2,
    Ring3,
    IOPL,   // rFlags.IOPL
    CR4_TSD,
    CR4_PCE
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionDefinitionFlags {
    pub tttn: Option<u8>,
    pub mem_format: Option<u8>
}

#[derive(Copy, Clone, Debug)]
pub struct OperandDescription {
    pub addressing_mode: OperandAddressingMode,
    pub operand_type: OperandType,
    pub fixed_operand: Option<FixedOperand>
}

impl OperandDescription {
    fn matches_operand(&self, operand: &Option<Operand>, mode: Mode, op1_size: Option<OperandSize>) -> bool {
        if !(match self.addressing_mode { // Check addressing mode
           OperandAddressingMode::A   => test_op(operand, |op| op.is_far_pointer() ),
           OperandAddressingMode::BA  => operand.is_none(),
           OperandAddressingMode::BB  => operand.is_none(),
           OperandAddressingMode::BD  => operand.is_none(),
           OperandAddressingMode::C   => test_direct(operand, |reg| reg.is_control() ),
           OperandAddressingMode::D   => test_direct(operand, |reg| reg.is_debug() ),
           OperandAddressingMode::E   => test_op(operand, |op| op.is_general() || op.is_memory() ),
           OperandAddressingMode::ES  => test_op(operand, |op| op.is_fpu() || op.is_memory() ),
           OperandAddressingMode::EST => test_op(operand, |op| op.is_fpu() ),
           OperandAddressingMode::F   => test_op(operand, |op| op.is_flags() ),
           OperandAddressingMode::G   => test_direct(operand, |reg| reg.is_general() ), 
           OperandAddressingMode::H   => test_direct(operand, |reg| reg.is_general() ),
           OperandAddressingMode::I   => test_op(operand, |op| op.is_literal() ),
           OperandAddressingMode::J   => test_op(operand, |op| op.is_literal() ),
           OperandAddressingMode::M   => test_op(operand, |op| op.is_memory() ),
           OperandAddressingMode::N   => test_direct(operand, |reg| reg.is_mmx() ),
           OperandAddressingMode::O   => test_op(operand, |op| op.is_fixed_memory() ),
           OperandAddressingMode::P   => test_direct(operand, |reg| reg.is_mmx() ),
           OperandAddressingMode::Q   => test_op(operand, |op| op.is_mmx() || op.is_memory() ),
           OperandAddressingMode::R   => test_op(operand, |op| op.is_general() ),
           OperandAddressingMode::S   => test_direct(operand, |reg| reg.is_segment()),
           OperandAddressingMode::SC  => panic!("SC not implemented."),
           OperandAddressingMode::T   => test_direct(operand, |reg| reg.is_test()),
           OperandAddressingMode::U   => test_direct(operand, |reg| reg.is_sse()),
           OperandAddressingMode::V   => test_direct(operand, |reg| reg.is_sse()),
           OperandAddressingMode::W   => test_op(operand, |op| op.is_sse() || op.is_memory()),
           OperandAddressingMode::X   => operand.is_none(),
           OperandAddressingMode::Y   => operand.is_none(),
           OperandAddressingMode::Z   => test_op(operand, |op| op.is_general() ),
           OperandAddressingMode::AVXMemBcst32Rm |
           OperandAddressingMode::AVXMemBcst64Rm => test_op(operand, |op| match *op {
               Operand::Direct(reg) => reg.is_avx(),
               Operand::Indirect(..) |
               Operand::IndirectDisplaced(..) |
               Operand::IndirectScaledIndexed(..) |
               Operand::IndirectScaledIndexedDisplaced(..) |
               Operand::IndirectScaledDisplaced(..) |
               Operand::AVXBroadcastIndirect(..) |
               Operand::AVXBroadcastIndirectDisplaced(..) |
               Operand::AVXBroadcastIndirectScaledIndexed(..) |
               Operand::AVXBroadcastIndirectScaledIndexedDisplaced(..) => true,
                _ => false
           }),
           OperandAddressingMode::AVXRegMaskedRm |
           OperandAddressingMode::AVXMaskedReg => test_op(operand, |op| match *op {
               Operand::Direct(reg) |
               Operand::AVXDestination(reg, ..) => reg.is_avx(),
               _ => false
           }),
           OperandAddressingMode::AVXVex |
           OperandAddressingMode::AVXImm8 |
           OperandAddressingMode::AVXRm => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_avx(),
                _ => false
           }),
           OperandAddressingMode::AVXMemRm => test_op(operand, |op| (op.is_avx() && !op.is_avx_op1()) || op.is_memory()),
           OperandAddressingMode::AVXReg => test_op(operand, |op| op.is_avx() && !op.is_avx_op1() && !op.is_memory()),
           OperandAddressingMode::AVXDestMemRm => test_op(operand, |op| op.is_memory()),
           OperandAddressingMode::MaskedMaskReg => test_op(operand, |op| match *op {
               Operand::Direct(reg) |
               Operand::AVXDestination(reg, ..) => reg.is_mask(),
               _ => false
           }),
           OperandAddressingMode::MaskReg => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => false
           }),
           OperandAddressingMode::MaskRm => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => false
           }),
           OperandAddressingMode::MaskMemRm => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => op.is_memory()
           }),
           OperandAddressingMode::MaskVex => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => false
           }),
           OperandAddressingMode::GeneralRm | 
           OperandAddressingMode::GeneralVex => test_direct(operand, |reg| reg.is_general()),
           OperandAddressingMode::BoundReg => test_direct(operand, |reg| reg.is_bounds()),
           OperandAddressingMode::BoundMemRm => test_direct(operand, |reg| reg.is_bounds())
               || test_op(operand, |op| op.is_memory() &&
                    if let Some(op_size) = op.size() {
                        match mode {
                            Mode::Protected => op_size == OperandSize::Qword,
                            Mode::Long => op_size == OperandSize::XMMword,
                            _ => false
                        }
                    } else { true }
               ),
           OperandAddressingMode::Fixed => true // Checking of fixed operands is done in operand type check
        }) { return false; } // Addressing mode mismatch

        if let Some(fixed_op) = self.fixed_operand {
            if operand.is_some() {
                if !(match fixed_op {
                    FixedOperand::AL        => test_direct(operand, |reg| reg == Reg::AL),
                    FixedOperand::AX        => test_direct(operand, |reg| reg == Reg::AX),
                    FixedOperand::ASized    => if let Some(Operand::Direct(reg)) = *operand {
                        match reg {
                            Reg::AX | Reg::EAX | Reg::RAX => true,
                            _ => false
                        }
                    } else { false },
                    FixedOperand::ASized32  => if let Some(Operand::Direct(reg)) = *operand {
                        match reg {
                            Reg::AL | Reg::AX | Reg::EAX => true,
                            _ => false
                        }
                    } else { false },
                    FixedOperand::EAX       => test_direct(operand, |reg| reg == Reg::EAX),
                    FixedOperand::CL        => test_direct(operand, |reg| reg == Reg::CL),
                    FixedOperand::DX        => test_direct(operand, |reg| reg == Reg::DX),
                    FixedOperand::CS        => test_direct(operand, |reg| reg == Reg::CS),
                    FixedOperand::DS        => test_direct(operand, |reg| reg == Reg::DS),
                    FixedOperand::ES        => test_direct(operand, |reg| reg == Reg::ES),
                    FixedOperand::FS        => test_direct(operand, |reg| reg == Reg::FS),
                    FixedOperand::GS        => test_direct(operand, |reg| reg == Reg::GS),
                    FixedOperand::SS        => test_direct(operand, |reg| reg == Reg::SS),
                    FixedOperand::ST1       => test_direct(operand, |reg| reg == Reg::ST1),
                    FixedOperand::AIndirectSizedDS      => test_indirect_seg(operand, SegmentReg::DS, |reg| reg == Reg::AX || reg == Reg::EAX || reg == Reg::RAX),
                    FixedOperand::DIIndirect            => test_indirect(operand, |r| reg_matches_mode(mode, r, Reg::DI, Reg::EDI, Reg::RDI)),
                    FixedOperand::SIIndirect            => test_indirect(operand, |r| reg_matches_mode(mode, r, Reg::SI, Reg::ESI, Reg::RSI)),
                    FixedOperand::DIIndirectSized       => test_indirect(operand, |r| r == Reg::DI || r == Reg::EDI || r == Reg::RDI),
                    FixedOperand::SIIndirectSized       => test_indirect(operand, |r| r == Reg::SI || r == Reg::ESI || r == Reg::RSI),
                    FixedOperand::BSizedALIndirectDS    => test_op(operand, |op| if let Operand::IndirectScaledIndexed(base, offset, scale, _, seg) = *op {
                        (base == Reg::BX || base == Reg::EBX || base == Reg::RBX) && (offset == Reg::AL) && (scale == RegScale::One) && seg.map(|s| s == SegmentReg::DS).unwrap_or(true)
                    } else {
                        false
                    }),
                    FixedOperand::SIIndirectDS          => test_indirect_seg(operand, SegmentReg::DS, |r| reg_matches_mode(mode, r, Reg::SI, Reg::ESI, Reg::RSI)), 
                    FixedOperand::SIIndirectSizedDS     => test_indirect_seg(operand, SegmentReg::DS, |r| r == Reg::SI || r == Reg::ESI || r == Reg::RSI),
                    FixedOperand::DIIndirectES          => test_indirect_seg(operand, SegmentReg::ES, |r| reg_matches_mode(mode, r, Reg::DI, Reg::EDI, Reg::RDI)),
                    FixedOperand::DIIndirectSizedES     => test_indirect_seg(operand, SegmentReg::ES, |r| r == Reg::DI || r == Reg::EDI || r == Reg::RDI),
                    FixedOperand::SPIndirectSizedSS     => test_indirect_seg(operand, SegmentReg::SS, |r| reg_matches_mode(mode, r, Reg::SP, Reg::ESP, Reg::RSP)),
                    FixedOperand::DIIndirectSizedDS     => test_indirect_seg(operand, SegmentReg::DS, |r| r == Reg::DI || r == Reg::EDI || r == Reg::RDI),
                    FixedOperand::Flags => test_direct(operand, |r| r.is_flags()),
                    FixedOperand::GDTR  => test_direct(operand, |r| r == Reg::GDTR),
                    FixedOperand::LDTR  => test_direct(operand, |r| r == Reg::LDTR),
                    FixedOperand::IDTR  => test_direct(operand, |r| r == Reg::IDTR),
                    FixedOperand::TR    => test_direct(operand, |r| r == Reg::TR),
                    FixedOperand::XCR   => test_direct(operand, |r| r == Reg::XCR),
                    FixedOperand::MSR   => test_direct(operand, |r| r == Reg::MSR),
                    FixedOperand::PMC   => test_direct(operand, |r| r == Reg::PMC),
                    FixedOperand::ST    => test_direct(operand, |r| r == Reg::ST),
                    FixedOperand::Literal8(val) => test_op(operand, |op| if let Operand::Literal8(v) = *op {
                        v == val
                    } else {
                        false
                    })
                }) { return false; } // Fixed operand mismatch
            } else {
                // Skip operand type check for implied fixed operands
                return true; 
            }
        }

        return match self.operand_type { // Test operand type
            OperandType::A      => if let Some(ref op) = *operand {
                if op.size().map(|o_s| o_s == OperandSize::Dword).unwrap_or(false) { return true; }
                else if op.size().map(|o_s| o_s == OperandSize::Qword).unwrap_or(false) { return true; }
                else if op.size().is_none() { true }
                else { false }
            } else { false },
            OperandType::B      => test_op_size_d(operand, OperandSize::Byte),
            OperandType::BCD    => test_op_size_d(operand, OperandSize::Tbyte),
            OperandType::BS     | 
            OperandType::BSQ    |
            OperandType::BSS    => operand.as_ref().and_then(|op| op.size().map(|o_s| o_s == OperandSize::Byte)).unwrap_or(true),
            OperandType::D      |
            OperandType::DO     |
            OperandType::DI     => test_op_size_d(operand, OperandSize::Dword) || (self.addressing_mode == OperandAddressingMode::P) || (self.addressing_mode == OperandAddressingMode::Q)
                                    | (self.addressing_mode == OperandAddressingMode::V) || (self.addressing_mode == OperandAddressingMode::W),
            OperandType::DQ     => test_op_size_d(operand, OperandSize::XMMword),
            OperandType::DQP    => test_op_size2_d(operand, OperandSize::Dword, OperandSize::Qword),
            OperandType::DR     => test_op(operand, |op| op.is_fpu() || op.is_memory() && op.size().map(|o_s| o_s == OperandSize::Qword).unwrap_or(true)),
            OperandType::DS     => operand.as_ref().and_then(|op| op.size().map(|o_s| o_s == OperandSize::Dword)).unwrap_or(true),
            OperandType::E      => test_op(operand, |op| op.is_memory()),
            OperandType::ER     => test_op_size_d(operand, OperandSize::Tbyte),
            OperandType::P      => if let Some(ref op) = *operand {
                if op.is_memory() {
                    test_op_size2(op, OperandSize::Dword, OperandSize::Fword)
                } else { match *op {
                    Operand::MemoryAndSegment16(..) |
                    Operand::MemoryAndSegment32(..) => true,
                    _ => false
                } }
            } else { false },
            OperandType::PI     => test_op_size_d(operand, OperandSize::Qword),
            OperandType::PD     => test_op_size_d(operand, OperandSize::XMMword),
            OperandType::PS     => test_op_size_d(operand, OperandSize::XMMword),
            OperandType::PSQ    => test_op(operand, |op| op.size().map(|o_s| o_s == OperandSize::Qword).unwrap_or(true) || op.is_sse()),
            OperandType::PT     => test_op_size_d(operand, OperandSize::Tbyte),
            OperandType::PTP    => if let Some(ref op) = *operand {
                if op.is_memory() {
                    op.size().map(|op_size| (op_size == OperandSize::Dword) || (op_size == OperandSize::Fword) || (op_size == OperandSize::Tbyte)).unwrap_or(true)
                } else { match *op {
                    Operand::MemoryAndSegment16(..) |
                    Operand::MemoryAndSegment32(..) => true,
                    // TODO MemoryAndSegment64
                    _ => false
                } }
            } else { false },
            OperandType::Q      => test_op_size_d(operand, OperandSize::Qword) || test_direct(operand, |reg| reg.is_sse()),
            OperandType::QI     => test_op_size_d(operand, OperandSize::Qword),
            OperandType::QP     => test_op_size_d(operand, OperandSize::Qword),
            OperandType::S      => if mode == Mode::Long { test_op_size_d(operand, OperandSize::Fword) } else { test_op_size_d(operand, OperandSize::Tbyte) },
            OperandType::SD     => test_op_size2_d(operand, OperandSize::XMMword, OperandSize::Qword),
            OperandType::SI     => test_op_size_d(operand, OperandSize::Dword),
            OperandType::SR     => test_op(operand, |op| op.is_fpu() || op.is_memory() && test_op_size(op, OperandSize::Dword)),
            OperandType::SS     => test_op_size2_d(operand, OperandSize::XMMword, OperandSize::Dword),
            OperandType::ST     => test_op(operand, |op| op.is_memory()),
            OperandType::STX    => test_op(operand, |op| op.is_memory()),
            OperandType::T      => test_op_size_d(operand, OperandSize::Tbyte),
            OperandType::V      => test_op_size2_d(operand, OperandSize::Word, OperandSize::Dword),
            OperandType::VDS    => test_op_size2_d(operand, OperandSize::Dword, OperandSize::Word),
            OperandType::VQ     => test_op_size2_d(operand, OperandSize::Word, OperandSize::Qword),
            OperandType::VQP    => test_op_size3_d(operand, OperandSize::Word, OperandSize::Dword, OperandSize::Qword),
            OperandType::VS     => test_op_size2_d(operand, OperandSize::Word, OperandSize::Dword),
            OperandType::W      |
            OperandType::WO     |
            OperandType::WI     => test_op_size_d(operand, OperandSize::Word),
            OperandType::XMM    => test_op_size_d(operand, OperandSize::XMMword),
            OperandType::YMM    => test_op_size_d(operand, OperandSize::YMMword),
            OperandType::ZMM    => test_op_size_d(operand, OperandSize::ZMMword),
            OperandType::XMMorYMM => test_op_size2_d(operand, OperandSize::XMMword, OperandSize::YMMword),
            OperandType::XMMorYMMorMem32 => test_op(operand, |op| (op.is_direct() && test_op_size2(op, OperandSize::XMMword, OperandSize::YMMword) || (op.is_memory() && test_op_size(op, OperandSize::Dword)))),
            OperandType::XMMorYMMorMemOrMem64 => test_op(operand, |op| (op.is_direct() && test_op_size2(op, OperandSize::XMMword, OperandSize::YMMword)) || (op.is_memory() && test_op_size3(op, OperandSize::XMMword, OperandSize::YMMword, OperandSize::Qword))),
            OperandType::XMMorMem32 => test_op(operand, |op| (op.is_direct() && test_op_size(op, OperandSize::XMMword)) || (op.is_memory() && test_op_size(op, OperandSize::Dword))),
            OperandType::XMMorMemOrMem32 => test_op(operand, |op| test_op_size(op, OperandSize::XMMword) || op.is_memory() && test_op_size2(op, OperandSize::Dword, OperandSize::XMMword)),
            OperandType::XMMorMemOrMem64 => test_op(operand, |op| test_op_size(op, OperandSize::XMMword) || op.is_memory() && test_op_size2(op, OperandSize::Qword, OperandSize::XMMword)),
            OperandType::XMMorMem64 => test_op(operand, |op| if op.get_broadcast_mode().is_none() {
                (op.is_direct() && test_op_size(op, OperandSize::XMMword)) || (op.is_memory() && test_op_size(op, OperandSize::Qword))
            } else { // Broadcast
                op.is_memory()
            }),
            OperandType::YMMorMemOrMem32 => test_op(operand, |op| test_op_size(op, OperandSize::YMMword) || op.is_memory() && test_op_size2(op, OperandSize::Dword, OperandSize::YMMword)),
            OperandType::YMMorMemOrMem64 => test_op(operand, |op| test_op_size(op, OperandSize::YMMword) || op.is_memory() && test_op_size2(op, OperandSize::Qword, OperandSize::YMMword)),
            OperandType::ZMMorMemOrMem64 => test_op(operand, |op| test_op_size(op, OperandSize::ZMMword) || op.is_memory() && test_op_size2(op, OperandSize::Qword, OperandSize::ZMMword)),
            OperandType::AVX    => { 
                if test_op(operand, |o| (o.is_avx() && o.is_direct()) || (o.is_memory() && o.get_broadcast_mode().is_none())) { test_op_size3_d(operand, OperandSize::XMMword, OperandSize::YMMword, OperandSize::ZMMword) }
                else if test_op(operand, |o| o.is_memory() && o.get_broadcast_mode().is_some()) { true }
                else { false }
            },
            OperandType::MaskReg => test_op(operand, |op| match *op {
                Operand::Direct(reg) |
                Operand::AVXDestination(reg, ..) => reg.is_mask(),
                _ => false
            }),
            OperandType::MaskOrMem8 => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => op.is_memory() && test_op_size(op, OperandSize::Byte)
            }),
            OperandType::MaskOrMem16 => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => op.is_memory() && test_op_size(op, OperandSize::Word)
            }),
            OperandType::MaskOrMem32 => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => op.is_memory() && test_op_size(op, OperandSize::Dword)
            }),
            OperandType::MaskOrMem64 => test_op(operand, |op| match *op {
                Operand::Direct(reg) => reg.is_mask(),
                _ => op.is_memory() && test_op_size(op, OperandSize::Qword)
            }),
            OperandType::Bound => test_direct(operand, |op| op.is_bounds()),
            OperandType::BoundOrMem => test_direct(operand, |reg| reg.is_bounds())
               || test_op(operand, |op| op.is_memory() && if let Some(op_size) = op.size() {
                    match mode {
                        Mode::Protected => op_size == OperandSize::Qword,
                        Mode::Long => op_size == OperandSize::XMMword,
                        _ => false
                    }
                } else { true }),
            OperandType::UnsizedMemory  => test_op(operand, |op| op.is_memory()),
            OperandType::FpuRegister => test_op(operand, |op| op.is_fpu()),
        }
    }

    fn get_size(&self, operand_size_prefix: bool, force_64: bool, vector_size: Option<OperandSize>, mode: Mode) -> Option<OperandSize> {
        match self.operand_type {
            OperandType::A => Some(if operand_size_prefix { OperandSize::Word } else { OperandSize::Dword }),
            OperandType::B => Some(OperandSize::Byte),
            OperandType::BCD => Some(OperandSize::Tbyte),
            OperandType::BS |
            OperandType::BSQ |
            OperandType::BSS => Some(OperandSize::Byte),
            OperandType::D | 
            OperandType::DO |
            OperandType::DI => Some(OperandSize::Dword),
            OperandType::DQ => Some(OperandSize::XMMword),
            OperandType::DQP => Some(if force_64 { OperandSize::Qword } else { OperandSize::Dword }),
            OperandType::DR => Some(OperandSize::Qword),
            OperandType::DS => Some(OperandSize::Dword),
            OperandType::E => None,
            OperandType::ER => Some(OperandSize::Tbyte),
            OperandType::P => Some(if operand_size_prefix { OperandSize::Fword } else { OperandSize::Word }),
            OperandType::PI => Some(OperandSize::Qword),
            OperandType::PD |
            OperandType::PS => Some(OperandSize::XMMword),
            OperandType::PSQ => Some(OperandSize::Qword),
            OperandType::PT => Some(OperandSize::Tbyte),
            OperandType::PTP => Some(if force_64 { OperandSize::Tbyte } else { if operand_size_prefix { OperandSize::Fword } else { OperandSize::Word }}),
            OperandType::Q |
            OperandType::QI |
            OperandType::QP => Some(OperandSize::Qword),
            OperandType::S => Some(if mode == Mode::Long { OperandSize::Tbyte } else { OperandSize::Fword }),
            OperandType::SD => Some(OperandSize::XMMword),
            OperandType::SI => Some(OperandSize::Dword),
            OperandType::SR => Some(OperandSize::Dword),
            OperandType::SS => Some(OperandSize::XMMword),
            OperandType::ST => None,
            OperandType::STX => None,
            OperandType::T => Some(OperandSize::Tbyte),
            OperandType::V |
            OperandType::VDS => Some(if operand_size_prefix { OperandSize::Word } else { OperandSize::Dword }),
            OperandType::VQ => Some(if operand_size_prefix { OperandSize::Word } else { OperandSize::Qword }),
            OperandType::VQP => Some(if force_64 { OperandSize::Qword } else { if operand_size_prefix { OperandSize::Word } else { OperandSize::Dword }}),
            OperandType::VS => Some(if operand_size_prefix { OperandSize::Word } else { OperandSize::Dword }),
            OperandType::W |
            OperandType::WI |
            OperandType::WO => Some(OperandSize::Word),
            OperandType::XMM => Some(OperandSize::XMMword),
            OperandType::YMM => Some(OperandSize::YMMword),
            OperandType::ZMM => Some(OperandSize::ZMMword),
            OperandType::XMMorYMM |
            OperandType::XMMorYMMorMem32 |
            OperandType::XMMorYMMorMemOrMem64 => vector_size,
            OperandType::XMMorMem32 |
            OperandType::XMMorMemOrMem32 |
            OperandType::XMMorMemOrMem64 |
            OperandType::XMMorMem64 => Some(OperandSize::XMMword),
            OperandType::YMMorMemOrMem32 |
            OperandType::YMMorMemOrMem64 => Some(OperandSize::YMMword),
            OperandType::ZMMorMemOrMem64 => Some(OperandSize::ZMMword),
            OperandType::AVX => vector_size,
            OperandType::MaskReg |
            OperandType::MaskOrMem8 |
            OperandType::MaskOrMem16 |
            OperandType::MaskOrMem32 |
            OperandType::MaskOrMem64 |
            OperandType::Bound |
            OperandType::BoundOrMem |
            OperandType::UnsizedMemory |
            OperandType::FpuRegister => None
        }
    }

    pub fn get_operand_size_prefix(&self, operand: &Option<Operand>, mode: Mode) -> Option<(bool, bool)> {
        match mode {
            Mode::Protected |
            Mode::Long => {
                if let Some(ref op) = *operand {
                    match self.operand_type {
                        OperandType::DO => Some((true, false)), // TODO
                        OperandType::P | // Should never be used in long mode? TODO
                        OperandType::PTP => Some(if let Some(op_size) = op.size() { 
                            match *op {
                                Operand::MemoryAndSegment16(..) => (false, true),
                                _ => {
                                    let is_dword = op_size == OperandSize::Dword;
                                    (!is_dword, is_dword)
                                }
                            }
                        } else { (true, true) }),
                        OperandType::V |
                        OperandType::VDS |
                        OperandType::VQ | 
                        OperandType::VQP => Some(op.size().map(|o_s| {
                                                let is_word = o_s == OperandSize::Word;
                                                (!is_word, is_word)
                                            }).unwrap_or((true, true))),
                        OperandType::WO => Some((false, true)),
                        _ => None
                    }
                } else if let Some(fixed_op) = self.fixed_operand {
                    // If there's no operand provided, but there's an implied fixed operand, it may
                    // require an operand prefix.
                    match self.operand_type {
                        OperandType::DO => Some((true, false)), // TODO
                        OperandType::WO => Some((false, true)),
                        _ => None
                    }
                } else { None }
            },
            Mode::Real => {
                if let Some(ref op) = *operand {
                    match self.operand_type {
                        OperandType::DO => Some((true, false)), // TODO
                        OperandType::PTP => Some(if let Some(op_size) = op.size() { 
                            match *op { // TODO?
                                Operand::MemoryAndSegment16(..) => (false, true),
                                _ => {
                                    let is_dword = op_size == OperandSize::Dword;
                                    (!is_dword, is_dword)
                                }
                            }
                        } else { (true, true) }),
                        OperandType::V |
                        OperandType::VDS |
                        OperandType::VQ | 
                        OperandType::VQP => Some(op.size().map(|o_s| {
                                                let is_dword = o_s == OperandSize::Dword;
                                                (!is_dword, is_dword)
                                            }).unwrap_or((true, true))),
                        OperandType::WO => Some((false, true)),
                        _ => None
                    }
                } else if let Some(fixed_op) = self.fixed_operand {
                    // If there's no operand provided, but there's an implied fixed operand, it may
                    // require an operand prefix.
                    match self.operand_type {
                        OperandType::DO => Some((true, false)), // TODO
                        OperandType::WO => Some((false, true)),
                        _ => None
                    }
                } else { None }
            }
        }
    }

    pub fn get_rex_w(&self, op: &Operand) -> Option<(bool, bool)> {
        let is_qword = op.size().map(|s| s == OperandSize::Qword);
        match self.operand_type {
            OperandType::DQP |
            OperandType::VQP => is_qword.map(|i| (!i, i)).or(Some((true, true))),
            OperandType::PTP => op.size().map(|s| s == OperandSize::Tbyte).map(|i| (!i, i)).or(Some((true, true))),
            OperandType::QP => Some((false, true)),
            _ => None
        }
    }

    pub fn get_vector_size(&self, op: &Operand) -> Option<OperandSize> {
        match self.operand_type {
            OperandType::PD |
            OperandType::PS |
            OperandType::SD |
            OperandType::SS |
            OperandType::XMM |
            OperandType::XMMorMem32 |
            OperandType::XMMorMemOrMem32 |
            OperandType::XMMorMemOrMem64 |
            OperandType::XMMorMem64 => Some(OperandSize::XMMword),
            OperandType::YMM |
            OperandType::YMMorMemOrMem32 |
            OperandType::YMMorMemOrMem64 => Some(OperandSize::YMMword),
            OperandType::ZMM |
            OperandType::ZMMorMemOrMem64 => Some(OperandSize::ZMMword),
            OperandType::XMMorYMM |
            OperandType::XMMorYMMorMemOrMem64 |
            OperandType::AVX => if op.get_broadcast_mode().is_none() { op.size() } else {
                if let Some(op_bits) = op.size().map(|o_s| o_s.bits() ) {
                    OperandSize::from_bits(op_bits * op.get_broadcast_mode().unwrap().get_multiplier())
                } else { None }
            },
            _ => None
        }
    }
}

// Helper methods to test for specific operand types
fn test_indirect_seg<F>(op: &Option<Operand>, seg: SegmentReg, predicate: F) -> bool
    where F: Fn(Reg) -> bool {
    test_op(op, |o| if let Operand::Indirect(reg, size, segment) = *o {
        predicate(reg) && segment.map(|s| s == seg).unwrap_or(true)
    } else {
        false
    })
}

fn test_op_size(op: &Operand, size: OperandSize) -> bool {
    op.size().map(|o_s| o_s == size).unwrap_or(true)
}

fn test_op_size2(op: &Operand, size1: OperandSize, size2: OperandSize) -> bool {
    op.size().map(|o_s| o_s == size1 || o_s == size2).unwrap_or(true)
}

fn test_op_size3(op: &Operand, size1: OperandSize, size2: OperandSize, size3: OperandSize) -> bool {
    op.size().map(|o_s| o_s == size1 || o_s == size2 || o_s == size3).unwrap_or(true)
}

fn test_op_size_d(op: &Option<Operand>, size: OperandSize) -> bool {
    op.as_ref().map(|ref o| test_op_size(o, size)).unwrap_or(true)
}

fn test_op_size2_d(op: &Option<Operand>, size1: OperandSize, size2: OperandSize) -> bool {
    op.as_ref().map(|ref o| test_op_size2(o, size1, size2)).unwrap_or(true)
}

fn test_op_size3_d(op: &Option<Operand>, size1: OperandSize, size2: OperandSize, size3: OperandSize) -> bool {
    op.as_ref().map(|ref o| test_op_size3(o, size1, size2, size3)).unwrap_or(true)
}

fn test_op<F>(op: &Option<Operand>, predicate: F) -> bool
    where F: Fn(&Operand) -> bool {
    op.as_ref().map(|o| predicate(o)).unwrap_or(false)
}

fn test_direct<F>(op: &Option<Operand>, predicate: F) -> bool 
    where F: Fn(Reg) -> bool {
    test_op(op, |o| if let Operand::Direct(reg) = *o { predicate(reg) } else { false })
}

fn test_indirect<F>(op: &Option<Operand>, predicate: F) -> bool 
    where F: Fn(Reg) -> bool {
    test_op(op, |o| if let Operand::Indirect(reg, ..) = *o { predicate(reg) } else { false })
}

fn reg_matches_mode(mode: Mode, reg: Reg, real_reg: Reg, prot_reg: Reg, long_reg: Reg) -> bool {
    match mode {
        Mode::Real => (real_reg == reg),
        Mode::Protected => (prot_reg == reg),
        Mode::Long => (long_reg == reg)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperandAddressingMode {
    A,      // Direct address
    BA,     // DS:EAX/RAX
    BB,     // DS:eBX+AL/rBX+AL
    BD,     // DS:eDI/RDI
    C,      // Reg selects control register
    D,      // Reg selects debug register
    E,      // ModR/M specifies general purpose register or memory location
    ES,     // ModR/M specifies FPU register or memory location
    EST,    // ModR/M specifies FPU register
    F,      // FLAGS register
    G,      // General purpose register
    H,      // R/M field always selects a general register
    I,      // Immediate data
    J,      // Relative offset added to IP
    M,      // ModR/M specifies memory only
    N,      // R/M field specifies MMX register
    O,      // No ModR/M byte, offset
    P,      // Reg selects MMX register
    Q,      // ModR/M specifies MMX or memory location
    R,      // Mod selects only general register
    S,      // Reg selects segment register
    SC,     // Stack operand
    T,      // Reg selects test register
    U,      // R/M selects XMM
    V,      // Reg selects XMM
    W,      // ModR/M specifies XMM or memory
    X,      // DS:ESI/RSI, long mode only supports 32/64 bit
    Y,      // ES:EDI/RDI, long mode only supports 32/64 bit
    Z,      // Least three significant bits select general register
    AVXVex, // AVX register encoded in Vex:vvvv
    AVXMemRm,   // AVX register or memory encoded in ModR/M:Rm
    AVXReg, // AVX register encoded in ModR/M:Reg
    AVXRm,  // AVX register encoded in ModR/M:Rm
    AVXMaskedReg,   // AVX register in ModR/M:Reg, with mask and merge modes
    AVXMemBcst32Rm,  // AVX register, AVXword memory, or 32-bit broadcast, encoded in ModR/M:Rm
    AVXMemBcst64Rm,  // AVX register, AVXword memory, or 64-bit broadcast, encoded in ModR/M:Rm
    AVXImm8,    // AVX register encoded in Imm8[7:4]
    AVXDestMemRm,   // AVX memory operand1 encoded in ModR/M:Rm
    AVXRegMaskedRm, // Masked AVX register encoded in ModR/M:Rm
    MaskedMaskReg,  // Masked mask register encoded in ModR/M:Reg
    MaskReg,    // Mask register encoded in ModR/M:Reg
    MaskRm,     // Mask register encoded in ModR/M:Rm
    MaskMemRm,  // Mask register or memory encoded in ModR/M:Rm
    MaskVex,    // Mask register encoded in Vex:vvvv
    GeneralVex, // General purpose register, encoded in vex
    GeneralRm,  // General purpose register, encoded in ModR/M:Rm
    BoundReg,   // Bounds register, encoded in ModR/M:Reg
    BoundMemRm, // Bounds register or memory, encoded in ModR/M:Rm
    Fixed,  // Fixed operand
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperandType {
    A,      // Two word/dword operands (memory)
    B,      // Byte
    BCD,    // Packed bcd
    BS,     // Byte (sign-extended to operand size)
    BSQ,    // Byte (sign-extended to qword)
    BSS,    // Byte (sign-extended to stack pointer size)
    D,      // Dword
    DO,     // Dword, force op size prefix?
    DI,     // Dword (fpu-integer)
    DQ,     // Dqword
    DQP,    // Dword/qword
    DR,     // Double real
    DS,     // Dword (sign-extended to qword)
    E,      // FPU environment
    ER,     // Extended real
    P,      // Pointer (32/48 bits)
    PI,     // MMX qword
    PD,     // Packed 128 bit doubles
    PS,     // Packed 128 bit singles
    PSQ,    // Packed 64 bit singles
    PT,     // 80-bit far pointer
    PTP,    // 32/48 bit pointer (based on operand size), or 80 bit far pointer (REX.W)
    Q,      // Qword
    QI,     // Qword (fpu-integer)
    QP,     // Qword (promoted by REX.W)
    S,      // 6-byte pseudo descriptor (10-byte in long mode)
    SD,     // Scalar element of 128-bit packed doubles
    SI,     // Dword integer register
    SR,     // Single real
    SS,     // Scalar element of 128-bit packed singles
    ST,     // FPU state
    STX,    // FPU+SIMD state
    T,      // 10-byte far pointer
    V,      // Word/dword (based on operand size)
    VDS,    // Word/dword (based on operand size, sign extended to qword)
    VQ,     // Word/qword (based on operand size)
    VQP,    // Word/dword (based on operand size, promoted by REX.W)
    VS,     // Word/dword (sign extended to stack pointer size)
    W,      // Word
    WI,     // Word (fpu-integer)
    WO,     // Word, force operand size prefix?
    XMM,    // XMMword
    YMM,    // YMMword
    ZMM,    // ZMMword
    XMMorYMM,   // XMMword or YMMword
    XMMorYMMorMem32,    // XMM/YMM or Dword Ptr
    XMMorYMMorMemOrMem64,   // XMM/YMM or XMM/YMMword Ptr or Qword Ptr
    XMMorMem32, // XMMword or Dword Ptr
    XMMorMemOrMem32, // XMM register or XMMword Ptr or Dword Ptr
    XMMorMemOrMem64, // XMM register or XMMword Ptr or Dword Ptr
    XMMorMem64, // XMMword or Qword Ptr
    YMMorMemOrMem32, // YMMword or Dword Ptr
    YMMorMemOrMem64, // YMMword or Qword Ptr
    ZMMorMemOrMem64, // ZMMword or Qword Ptr
    AVX,    // XMMword, YMMword, or ZMMword
    MaskReg,    // Mask register
    MaskOrMem8, // Mask register or Byte Ptr
    MaskOrMem16, // Mask register or Word Ptr
    MaskOrMem32, // Mask register or Dword Ptr
    MaskOrMem64, // Mask register or Qword Ptr
    Bound,      // Bound register
    BoundOrMem, // Bound register or memory
    UnsizedMemory,  // Used when only address of memory is used (i.e. LEA)
    FpuRegister,
}

#[derive(Clone, Copy, Debug)]
pub enum FixedOperand {
    AL,
    AX,
    ASized,    // AX/EAX/RAX
    ASized32,  // AX/EAX
    EAX,
    CL,
    DX,
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,
    ST1,
    AIndirectSizedDS,
    DIIndirect,
    SIIndirect,
    DIIndirectSized,
    SIIndirectSized,
    BSizedALIndirectDS,
    SIIndirectDS,
    SIIndirectSizedDS,
    DIIndirectES,
    DIIndirectSizedES,
    SPIndirectSizedSS,
    DIIndirectSizedDS,
    Flags,
    GDTR,
    LDTR,
    IDTR,
    TR,
    XCR,
    MSR,
    PMC,
    ST,
    Literal8(u8)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpSize64Behavior {
    Normal,
    Force64,
    Force64EvexOnly
}

impl Default for OpSize64Behavior {
    fn default() -> OpSize64Behavior {
        OpSize64Behavior::Normal
    }
}
