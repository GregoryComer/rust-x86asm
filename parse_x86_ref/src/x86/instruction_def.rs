/*
 *  Instruction format and definitions from ref.x86asm.net
 */

use std::collections::HashMap;
use std::sync::Mutex;
use std::io::Write;
use x86::{ Instruction, Operand, Mnemonic, Mode, Reg, OperandSize, RegScale, SegmentReg, ProcessorLevel, BroadcastMode };
use x86::instruction_buffer::{ InstructionBuffer, ImmediateValue, Prefix1, Prefix2 };
use itertools::Itertools;

lazy_static! {
    static ref INSTR_MNEMONIC_MAP : Mutex<HashMap<Mnemonic, Vec<&'static InstructionDefinition>>> = { Mutex::new(HashMap::new()) };
}

#[derive(Debug)]
pub enum InstructionEncodingError {
    InvalidMnemonic,
    NoEncoding,
    WriteFailed,
    MismatchedSize,
    AmbiguousSize
}

#[derive(Copy, Clone, Debug)]
enum InstructionMatch {
    Match(Option<OperandSize>),
    AmbiguousSize,
    NoMatch
}

#[derive(Debug)]
enum OperandMatch {
    Match(Option<OperandSize>),
    AmbiguousSize2(OperandSize, OperandSize),
    AmbiguousSize3(OperandSize, OperandSize, OperandSize),
    NoMatch
}

pub fn find_instruction_def(instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<&InstructionDefinition, InstructionEncodingError> {
    if let Some(list) = INSTR_MNEMONIC_MAP.lock().unwrap().get(&instr.mnemonic) {
        let encoding_matches = list.iter()
            .map(|enc| (enc, enc.matches_instruction(instr, mode, proc_level)) );
        let mut max: Option<(u32, _)> = None;
        let mut op_size = None;
        for enc_match in encoding_matches {
            println!("Match: {:x} ({:?}), {:?}", enc_match.0.primary_opcode, enc_match.0.mnemonic, enc_match.1);
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

pub fn load_instructions() {
    /*let mut map = INSTR_MNEMONIC_MAP.lock().unwrap();
    for instr in INSTR_DEFS.iter() {
        if !map.contains_key(&instr.mnemonic) {
            let new_list = Vec::new();
            map.insert(instr.mnemonic, new_list);
        };
        let mut list = map.get_mut(&instr.mnemonic).unwrap();
        list.push(&instr);
    }*/
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
    pub has_r: bool,
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
    pub source: Option<OperandDescription>,
    pub source2: Option<OperandDescription>,
    pub source3: Option<OperandDescription>,
    pub destination: Option<OperandDescription>
}

impl InstructionDefinition {
    pub fn encode<W>(&self, writer: &mut W, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<usize, InstructionEncodingError>
        where W : Write {
        match mode {
            Mode::Real => panic!("Not implemented."),
            Mode::Protected => self.encode32(writer, instr, mode, proc_level),
            Mode::Long => panic!("Not implemented."),
            Mode::SystemManagement => panic!("Not implemented.")
        }
    }

    fn encode32<W>(&self, writer: &mut W, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> Result<usize, InstructionEncodingError>
        where W : Write {
        let mut buffer: InstructionBuffer = Default::default(); 

        println!("encode32() {:?}", self);

        let operand_pairs = [(&self.source, &instr.source),
            (&self.source2, &instr.source2),
            (&self.source3, &instr.source3),
            (&self.destination, &instr.destination)];

        if instr.flags.lock { buffer.prefix1 = Some(Prefix1::Lock); }
        // TODO Rep prefixes

		buffer.is_two_byte_opcode = self.is_two_byte_opcode;
		buffer.primary_opcode = self.primary_opcode;
		buffer.secondary_opcode = self.secondary_opcode;
        buffer.fixed_prefix = self.fixed_prefix;

        // Instruction size prefix
        if self.force_op_size_prefix {
            buffer.operand_size_prefix = true;
        } else if let Some(pref) = operand_pairs.iter().filter_map(|tuple| tuple.0.and_then(
                    |op_def| op_def.get_operand_size_prefix(tuple.1, mode) ) ).next() {
            buffer.operand_size_prefix = pref;
        }

        // Address size prefix
        if let Some(pref) = operand_pairs.iter().filter_map(|tuple| tuple.0.and_then(
                    |op_def| tuple.1.as_ref().and_then(|ref op| op_def.get_address_size_prefix(op, mode) ) ) ).next() {
            buffer.address_size_prefix = pref;
        }
       
        if let Some(op) = self.source { op.encode(&mut buffer, &instr.source, mode, proc_level)?; }
        if let Some(op) = self.source2 { op.encode(&mut buffer, &instr.source2, mode, proc_level)?; }
        if let Some(op) = self.source3 { op.encode(&mut buffer, &instr.source3, mode, proc_level)?; }
        if let Some(op) = self.destination { op.encode(&mut buffer, &instr.destination, mode, proc_level)?; }

        if let Some(op_ext) = self.opcode_ext {
            // Seems like the source document uses secondary opcodes as a special case of mod/rm
            // bytes in some cases? TODO Look into this more
            if self.secondary_opcode.is_none() {
                buffer.mod_rm_reg = Some(op_ext);
            }
        }

		//println!("Instruction Buffer: {:?}", buffer);

        buffer.write(writer)
    }

    pub fn matches_instruction(&self, instr: &Instruction, mode: Mode, proc_level: ProcessorLevel) -> InstructionMatch {
       let operand_pairs = [(&self.source, &instr.source),
        (&self.source2, &instr.source2),
        (&self.source3, &instr.source3),
        (&self.destination, &instr.destination)];

       // Check mode
       if !(match mode {
           Mode::Real => (self.mode == Mode::Real),
           Mode::Protected => (self.mode == Mode::Real || self.mode == Mode::Protected),
           Mode::Long => (self.mode == Mode::Real || self.mode == Mode::Protected || self.mode == Mode::Long) && self.valid_in_long_mode,
           Mode::SystemManagement => self.mode == Mode::SystemManagement
		} &&

       // Check flags
       (!instr.flags.lock || self.can_lock) &&

       // Check processor level
       self.proc_start.map(|p| proc_level >= p).unwrap_or(true) &&
       self.proc_end.map(|p| proc_level <= p).unwrap_or(true) &&

       // Check operand size
       operand_pairs.iter()
            .filter_map(|tuple| tuple.0.and_then(
                    |op_def| op_def.get_operand_size_prefix(tuple.1, mode) ) )
            .fold(Ok(None), |acc, v| {
                acc.and_then(|acc_val|
                    if acc_val.map(|a| a == v).unwrap_or(true) { Ok(Some(v)) } 
                    else { Err(InstructionEncodingError::MismatchedSize) }
                )}).is_ok() &&

       // Check addressing mode
       operand_pairs.iter()
            .filter_map(|tuple| tuple.0.and_then(
                    |op_def| tuple.1.as_ref().and_then(|ref op| op_def.get_address_size_prefix(op, mode) ) ) )
            .fold(Ok(None), |acc, v| {
                acc.and_then(|acc_val|
                    if acc_val.map(|a| a == v).unwrap_or(true) { Ok(Some(v)) } 
                    else { Err(InstructionEncodingError::MismatchedSize) }
                )}).is_ok()
    	) { return InstructionMatch::NoMatch; }
        
        // Check operands and make sure size is unambiguous
        let mut op_matches = operand_pairs.iter()
            .map(|tuple| (tuple.0, tuple.1, match *tuple.0 {
                // If there's an expected operand, test the provided one against it
                Some(op_def) => Some(op_def.matches_operand(tuple.1, mode, 
                    // Some operations use sign extend to dest to mean sign extend to other source
                    instr.destination.as_ref().map(|d| d.size() ).or(instr.source.as_ref().map(|s| s.size() ) ))),
                // If there's no operand expected, but one was provided, it's an error
                None => tuple.1.as_ref().map(|_| OperandMatch::NoMatch )
            }) ).inspect(|match_tuple| println!("Operand Match: {:?}", match_tuple) );

        let mut filtered_op_matches = op_matches.filter_map(|tuple| tuple.2);
        if let Some(op_match) = filtered_op_matches.next() {
            // Tuple of possible operand sizes for first operand
            let seed = match op_match {
                OperandMatch::Match(maybe_size) => Ok((maybe_size, None, None)),
                OperandMatch::AmbiguousSize2(size1, size2) => Ok((Some(size1), Some(size2), None)),
                OperandMatch::AmbiguousSize3(size1, size2, size3) => Ok((Some(size1), Some(size2), Some(size3))),
                OperandMatch::NoMatch => Err(())
            };

            println!("Seed: {:?}", seed);

            fn op_test_helper<F>(acc: (Option<OperandSize>, Option<OperandSize>, Option<OperandSize>), test_proc: F) ->
                (Option<OperandSize>, Option<OperandSize>, Option<OperandSize>)
                where F: Fn(OperandSize) -> bool {
                fn inner_helper<F>(v: Option<OperandSize>, test_proc: &F) -> Option<OperandSize>
                    where F: Fn(OperandSize) -> bool {
                    v.and_then(|val| if test_proc(val) { v } else { None })
                }

                (inner_helper(acc.0, &test_proc),
                 inner_helper(acc.1, &test_proc),
                 inner_helper(acc.2, &test_proc))
            }

            fn op_test_helper2(acc: (Option<OperandSize>, Option<OperandSize>, Option<OperandSize>), op_match: &OperandMatch) -> Result<(Option<OperandSize>, Option<OperandSize>, Option<OperandSize>), ()> {
                match *op_match {
                    OperandMatch::Match(size) => Ok(op_test_helper(acc, |o| size.map(|s| s == o).unwrap_or(true) )),
                    OperandMatch::AmbiguousSize2(size1, size2) => Ok(op_test_helper(acc, |o| o == size1 || o == size2)),
                    OperandMatch::AmbiguousSize3(size1, size2, size3) => Ok(op_test_helper(acc, |o| o == size1 || o == size2 || o == size3)),
                    OperandMatch::NoMatch => Err(())
                }
            }

            let common_op_types_res = filtered_op_matches.fold(seed, |acc_res, ref op_match| {
                println!("acc_res: {:?}, op_match: {:?}", acc_res, op_match);
                acc_res.and_then(|acc| op_test_helper2(acc, op_match) )
            });

            if let Ok(common_op_types) = common_op_types_res {
                let type_array = [common_op_types.0, common_op_types.1, common_op_types.2];
                let mut possible_types = type_array.iter().filter_map(|&x| x);
                if let Some(t) = possible_types.next() {
                    if let None = possible_types.next() {
                        InstructionMatch::Match(Some(t))
                    } else {
                        InstructionMatch::AmbiguousSize
                    }
                } else {
                    InstructionMatch::Match(None)
                }
            } else { InstructionMatch::NoMatch }
        } else { // No operands
            InstructionMatch::Match(None)
        }
    }
    
    // Determines which encoding is used when multiple are possible. Higher value means
    // instruction is more specific (i.e. one byte int 3) and should be chosen. This
    // metric could likely be improved in the future.
    fn get_specificity(&self) -> u32 {
        // TODO Prefer 64-bit encodings (i.e. PUSH 50)
        let mut score = 0;

        fn test_op(op: &Option<OperandDescription>) -> bool {
            if let Some(desc) = *op {
                desc.fixed_operand.is_some()
            } else {
                false 
            }
        }
        
        if test_op(&self.source) { score += 1; } 
        if test_op(&self.source2) { score += 1; } 
        if test_op(&self.source3) { score += 1; } 
        if test_op(&self.destination) { score += 1; } 

        score
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
    fn encode(&self, buffer: &mut InstructionBuffer, op: &Option<Operand>, mode: Mode, proc_level: ProcessorLevel) -> Result<(), InstructionEncodingError> {
        println!("encode() op: {:?}, addr: {:?}, type: {:?}", op, self.addressing_mode, self.operand_type);
        match self.addressing_mode {
            OperandAddressingMode::A => { // Direct memory address w/ segment selector
                buffer.immediate = Some(match *op.as_ref().expect("Missing operand.") {
                    Operand::MemoryAndSegment16(seg, addr) => ImmediateValue::MemoryAndSegment16(seg, addr),
                    Operand::MemoryAndSegment32(seg, addr) => ImmediateValue::MemoryAndSegment32(seg, addr),
                    _ => panic!("Invalid operand.")
                })
            },
            OperandAddressingMode::BA => {}, // DS:rAX
            OperandAddressingMode::BB => {}, // DS:rBX+AL
            OperandAddressingMode::BD => {}, // DS:rDI
            OperandAddressingMode::C => { // Reg field is control register
                if let Some(Operand::Direct(reg)) = *op {
                    encode_reg_direct(buffer, reg);
                } else { panic!("Invalid operand."); }
            },
            OperandAddressingMode::D => { // Reg field is 
                if let Some(Operand::Direct(reg)) = *op {
                    encode_reg_direct(buffer, reg);
                } else { panic!("Invalid operand."); }
            },
            OperandAddressingMode::E |     // Rm is general/mem
            OperandAddressingMode::ES => { // Rm is fpu/mem
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct(buffer, reg),
                    Operand::Indirect(reg, ..) => encode_rm_indirect(buffer, reg),
                    Operand::IndirectDisplaced(reg, disp, ..) => encode_rm_indirect_displaced(buffer, reg, disp),
                    Operand::IndirectScaledIndexed(base, index, scale, ..) => encode_rm_indirect_scaled_indexed(buffer, base, index, scale),
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, .. ) => encode_rm_indirect_scaled_indexed_displaced(buffer, base, index, scale, disp),
                    Operand::Memory(addr, ..) => encode_rm_memory(buffer, addr),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::EST => { // Rm is fpu
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::F => {}, // Flags
            OperandAddressingMode::G => {   // Reg is general
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_reg_direct(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::H => { // Rm is general (ignore mod)
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct_no_mod(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::I => { // Immediate
               buffer.add_immediate(match *op.as_ref().expect("Missing operand.") {
                   Operand::Literal8(n) => ImmediateValue::Literal8(n),
                   Operand::Literal16(n) => ImmediateValue::Literal16(n),
                   Operand::Literal32(n) => ImmediateValue::Literal32(n),
                   Operand::Literal64(n) => ImmediateValue::Literal64(n),
                   _ => panic!("Invalid operand.")
               });
            },
            OperandAddressingMode::J => { // Relative offset
                // TODO Maybe should factor in instruction size here? Some assemblers do. See
                // relative call instructions (offset specified from next instruction).
                buffer.immediate = Some(match *op.as_ref().expect("Missing operand") {
                    Operand::Literal8(n) => ImmediateValue::Literal8(n),
                   Operand::Literal16(n) => ImmediateValue::Literal16(n),
                   Operand::Literal32(n) => ImmediateValue::Literal32(n),
                   Operand::Literal64(n) => ImmediateValue::Literal64(n),
                   _ => panic!("Invalid operand.")
                });
            },
            OperandAddressingMode::M => { // Rm is memory
                println!("Encoding M");
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Indirect(reg, ..) => encode_rm_indirect(buffer, reg),
                    Operand::IndirectDisplaced(reg, disp, ..) => encode_rm_indirect_displaced(buffer, reg, disp),
                    Operand::IndirectScaledIndexed(base, index, scale, ..) => encode_rm_indirect_scaled_indexed(buffer, base, index, scale),
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, .. ) => encode_rm_indirect_scaled_indexed_displaced(buffer, base, index, scale, disp),
                    Operand::Memory(addr, ..) => encode_rm_memory(buffer, addr),
                    Operand::MemoryAndSegment16(seg, addr) => buffer.immediate = Some(ImmediateValue::MemoryAndSegment16(seg, addr)),
                    Operand::MemoryAndSegment32(seg, addr) => buffer.immediate = Some(ImmediateValue::MemoryAndSegment32(seg, addr)),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::N => { // Rm is mmx
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::O => { // Offset
                panic!("Not implemented."); // Uses relative offsets
            },
            OperandAddressingMode::P => { // Reg is mmx
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_reg_direct(buffer, reg),
                   _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::Q => { // Rm is mmx or memory
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Indirect(reg, ..) => encode_rm_indirect(buffer, reg),
                    Operand::IndirectDisplaced(reg, disp, ..) => encode_rm_indirect_displaced(buffer, reg, disp),
                    Operand::IndirectScaledIndexed(base, index, scale, ..) => encode_rm_indirect_scaled_indexed(buffer, base, index, scale),
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, .. ) => encode_rm_indirect_scaled_indexed_displaced(buffer, base, index, scale, disp),
                    Operand::Memory(addr, ..) => encode_rm_memory(buffer, addr),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::R => { // Mod is general
                match *op.as_ref().expect("Missing operand.") {
                    // Source document says that this should mean that a general purpose register
                    // is encoded in the mod field. I think this is a typo, as it doesn't match
                    // Intel docs. Should be the r/m field? (See mov to/from control register)
                    Operand::Direct(reg) => encode_rm_direct(buffer, reg),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::S => { // Reg is segment
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_reg_direct(buffer, reg),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::SC => {}, // Stack operand
            OperandAddressingMode::T => { // Reg is test
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_reg_direct(buffer, reg),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::U => { // Rm is XMM
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_rm_direct(buffer, reg),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::V => { // Reg is XMM
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Direct(reg) => encode_reg_direct(buffer, reg),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::W => { // Rm is XMM/mem
                match *op.as_ref().expect("Missing operand.") {
                    Operand::Indirect(reg, ..) => encode_rm_indirect(buffer, reg),
                    Operand::IndirectDisplaced(reg, disp, ..) => encode_rm_indirect_displaced(buffer, reg, disp),
                    Operand::IndirectScaledIndexed(base, index, scale, ..) => encode_rm_indirect_scaled_indexed(buffer, base, index, scale),
                    Operand::IndirectScaledIndexedDisplaced(base, index, scale, disp, .. ) => encode_rm_indirect_scaled_indexed_displaced(buffer, base, index, scale, disp),
                    Operand::Memory(addr, ..) => encode_rm_memory(buffer, addr),
                    _ => panic!("Invalid operand.")
                }
            },
            OperandAddressingMode::X => {}, // DS:rSI
            OperandAddressingMode::Y => {}, // ES:rDI
            OperandAddressingMode::Z => {
                if let Some(Operand::Direct(reg)) = *op {
                    buffer.opcode_add = Some(reg.get_reg_code());
                } else { panic!("Invalid operand."); }
            }, // Last 3 bits of opcode is general reg
            OperandAddressingMode::Fixed => {}
        }

        Ok(())
    }

    fn matches_operand(&self, operand: &Option<Operand>, mode: Mode, dest_size: Option<OperandSize>) -> OperandMatch {
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
           OperandAddressingMode::Fixed => true // Checking of fixed operands is done in operand type check
        }) { println!("Addressing mode mismatch."); return OperandMatch::NoMatch; }

        if let Some(fixed_op) = self.fixed_operand {
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
            }) { println!("Fixed operand mismatch."); return OperandMatch::NoMatch; }
        }

        match self.operand_type { // Test operand type
            OperandType::A      => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Dword { OperandMatch::Match(Some(OperandSize::Word)) }
                else if op.size() == OperandSize::Qword { OperandMatch::Match(Some(OperandSize::Dword)) }
                else { OperandMatch::NoMatch }
            } else { OperandMatch::NoMatch },
            OperandType::B      => test_op_size(operand, OperandSize::Byte),
            OperandType::BCD    => test_op_size(operand, OperandSize::Tbyte),
            OperandType::BS     => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Byte && dest_size.is_some() { OperandMatch::Match(dest_size) } // Sign extend to dest operand size
                else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::BSQ    => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Byte { OperandMatch::Match(Some(OperandSize::Qword)) } // Sign extend to 64 bits
                else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::BSS    => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Byte { OperandMatch::Match(Some(mode.pointer_size())) } // Sign extend to pointer size
                else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::C      => test_op_size2(operand, OperandSize::Byte, OperandSize::Word),
            OperandType::D | 
            OperandType::DO     => test_op_size(operand, OperandSize::Dword),
            OperandType::DI     => test_op_size(operand, OperandSize::Dword),
            OperandType::DQ     => test_op_size(operand, OperandSize::XMMword),
            OperandType::DQP    => test_op_size2(operand, OperandSize::Dword, OperandSize::Qword),
            OperandType::DR     =>  if test_op(operand, |op| op.is_fpu() || op.is_memory() && op.size() == OperandSize::Qword) { 
                    OperandMatch::Match(Some(OperandSize::Qword))
                } else { OperandMatch::NoMatch },
            OperandType::DS     => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Dword { OperandMatch::Match(Some(OperandSize::Qword)) } // Sign extend to 64 bits
                else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::E      => match_helper_none(test_op(operand, |op| op.is_memory())),
            OperandType::ER     => test_op_size(operand, OperandSize::Tbyte),
            OperandType::P      => match *operand {
                Some(Operand::MemoryAndSegment16(..)) => OperandMatch::Match(Some(OperandSize::Dword)),
                Some(Operand::MemoryAndSegment32(..)) => OperandMatch::Match(Some(OperandSize::Fword)),
                _ => OperandMatch::NoMatch
            }, 
            OperandType::PI     => test_op_size(operand, OperandSize::Qword),
            OperandType::PD     => test_op_size(operand, OperandSize::XMMword),
            OperandType::PS     => test_op_size(operand, OperandSize::XMMword),
            OperandType::PSQ    => test_op_size(operand, OperandSize::Qword),
            OperandType::PT     => test_op_size(operand, OperandSize::Tbyte),
            OperandType::PTP    => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Dword { OperandMatch::Match(Some(OperandSize::Dword)) }
                else if op.size() == OperandSize::Fword { OperandMatch::Match(Some(OperandSize::Fword)) }
                else if op.size() == OperandSize::Byte { OperandMatch::Match(Some(OperandSize::Tbyte)) }
                else { OperandMatch::NoMatch }
            } else { OperandMatch::NoMatch },
            OperandType::Q      => test_op_size(operand, OperandSize::Qword),
            OperandType::QI     => test_op_size(operand, OperandSize::Qword),
            OperandType::QP     => test_op_size(operand, OperandSize::Qword),
            OperandType::S      => if mode == Mode::Long { test_op_size(operand, OperandSize::Fword) } else { test_op_size(operand, OperandSize::Tbyte) },
            OperandType::SD     => test_op_size(operand, OperandSize::XMMword),
            OperandType::SI     => test_op_size(operand, OperandSize::Dword),
            OperandType::SR     => if test_op(operand, |op| op.is_fpu() || op.is_memory() && op.size() == OperandSize::Dword) { 
                    OperandMatch::Match(Some(OperandSize::Dword))
                } else { OperandMatch::NoMatch },
            OperandType::SS     => test_op_size(operand, OperandSize::XMMword),
            OperandType::ST     => match_helper_none(test_op(operand, |op| op.is_memory())),
            OperandType::STX    => match_helper_none(test_op(operand, |op| op.is_memory())),
            OperandType::T      => test_op_size(operand, OperandSize::Tbyte),
            OperandType::V      => test_op_size2(operand, OperandSize::Word, OperandSize::Dword),
            OperandType::VDS    =>  if let Some(ref op) = *operand {
                if op.size() == OperandSize::Dword || op.size() == OperandSize::Word { 
                    if dest_size.map(|d| d == OperandSize::Qword).unwrap_or(false) { OperandMatch::Match(Some(OperandSize::Qword)) } // Sign extend to 64 bits
                    else { OperandMatch::Match(Some(op.size())) }
                } else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::VQ     => test_op_size2(operand, OperandSize::Word, OperandSize::Qword),
            OperandType::VQP    => test_op_size3(operand, OperandSize::Word, OperandSize::Dword, OperandSize::Qword),
            OperandType::VS     => if let Some(ref op) = *operand {
                if op.size() == OperandSize::Word || op.size() == OperandSize::Dword { OperandMatch::Match(Some(mode.pointer_size())) } // Sign extend to pointer size
                else { OperandMatch::NoMatch }
            } else { OperandMatch:: NoMatch },
            OperandType::W |
            OperandType::WO     => test_op_size(operand, OperandSize::Word),
            OperandType::WI     => test_op_size(operand, OperandSize::Word),
            OperandType::UnsizedMemory  => match_helper_none(test_op(operand, |op| op.is_memory())),
            OperandType::FpuRegister => match_helper_none(test_op(operand, |op| op.is_fpu())),
        }
    }

    // Returns a list of possible operand sizes.
    // TODO This function is incomplete. Need to decide how to handle
    // sign extension and mode dependant sizes. Not really a problem
    // right now, since it's only used to determine whether or not to
    // compare operand sizes.
    fn get_possible_sizes(&self) -> &'static [OperandSize] {
        // Would be nice if I could reference static arrays inline
        static sizes_word: &'static [OperandSize] = &[OperandSize::Word];
        static sizes_dword_qword: &'static [OperandSize] = &[OperandSize::Dword, OperandSize::Qword];
        static sizes_byte: &'static [OperandSize] = &[OperandSize::Byte];
        static sizes_byte_word: &'static [OperandSize] = &[OperandSize::Byte, OperandSize::Word];
        static sizes_dword: &'static [OperandSize] = &[OperandSize::Dword];
        static sizes_xmmword: &'static [OperandSize] = &[OperandSize::XMMword];
        static sizes_qword: &'static [OperandSize] = &[OperandSize::Qword];
        static sizes_dword_fword_tbyte: &'static [OperandSize] = &[OperandSize::Dword, OperandSize::Fword, OperandSize::Tbyte];
        static sizes_word_dword: &'static [OperandSize] = &[OperandSize::Word, OperandSize::Dword];
        static sizes_word_qword: &'static [OperandSize] = &[OperandSize::Word, OperandSize::Qword];

        match self.operand_type {
            OperandType::A   => sizes_dword_qword,
            OperandType::B   => sizes_byte,
            OperandType::C   => sizes_byte_word,
            OperandType::D   => sizes_dword,
            OperandType::DQ |
            OperandType::PD |
            OperandType::PS |
            OperandType::SD  => sizes_xmmword,
            OperandType::PSQ |
            OperandType::Q   => sizes_qword,
            OperandType::DQP => sizes_dword_qword,
            OperandType::PTP => sizes_dword_fword_tbyte,
            OperandType::V |
            OperandType::VQP => sizes_word_dword,
            OperandType::VQ  => sizes_word_qword,
            OperandType::W   => sizes_word,
            _ => &[]
        }
    }
    
    // TODO Ensure this works in real and long mode
    fn get_operand_size_prefix(&self, operand: &Option<Operand>, mode: Mode) -> Option<bool> {
        if let Some(ref op) = *operand {
            if let Some(fixed_op) = self.fixed_operand {
                match fixed_op {
                    FixedOperand::ASized32 => Some(op.size() == OperandSize::Word),
                    _ => None
                }
            } else {
                match self.operand_type {
                    // OperandType::C isn't used
                    OperandType::P if mode == Mode::Protected => // Should never be used in long mode
                        Some(if let Operand::MemoryAndSegment16(..) = *op { true } else { false }),
                    OperandType::PTP => 
                        Some(if let Operand::MemoryAndSegment16(..) = *op { true} else { false }),
                    OperandType::V |
                    OperandType::VDS |
                    OperandType::VQ | 
                    OperandType::VQP => Some(op.size() == OperandSize::Word),
                    _ => None
                } 
            }
        } else if let Some(fixed_op) = self.fixed_operand {
            // If there's no operand provided, but there's an implied fixed operand, it may
            // require an operand prefix.
            match fixed_op {
                FixedOperand::AX |
                FixedOperand::DX => Some(true),
                _ => None
            }
        } else { None }
    }

    // TODO Ensure this works in real and long mode
    fn get_address_size_prefix(&self, op: &Operand, mode: Mode) -> Option<bool> {
        match mode {
            Mode::Long => {
                panic!("Unimplemented.");
                // TODO
            },
            Mode::Protected => match *op {
                Operand::Indirect(reg, ..) |
                Operand::IndirectDisplaced(reg, ..) |
                Operand::IndirectScaledIndexed(reg, ..) |
                Operand::IndirectScaledIndexedDisplaced(reg, ..) =>
                    Some(reg.size() == OperandSize::Word),
                _ => None
            },
            Mode::Real => {
                panic!("Unimplemented.");
                // TODO
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

fn test_op_size(op: &Option<Operand>, size: OperandSize) -> OperandMatch {
    if let Some(ref o) = *op {
        if o.size() == size { OperandMatch::Match(Some(size)) } else { OperandMatch::NoMatch }
    } else { OperandMatch::NoMatch }
}

fn test_op_size2(op: &Option<Operand>, size1: OperandSize, size2: OperandSize) -> OperandMatch {
    if let Some(ref o) = *op {
        if o.size() == size1 { OperandMatch::Match(Some(size1)) }
        else if o.size() == size2 { OperandMatch::Match(Some(size2)) } 
        else { OperandMatch::NoMatch }
    } else { OperandMatch::NoMatch }
}

fn test_op_size3(op: &Option<Operand>, size1: OperandSize, size2: OperandSize, size3: OperandSize) -> OperandMatch {
    if let Some(ref o) = *op {
        if o.size() == size1 { OperandMatch::Match(Some(size1)) }
        else if o.size() == size2 { OperandMatch::Match(Some(size2)) } 
        else if o.size() == size3 { OperandMatch::Match(Some(size3)) } 
        else { OperandMatch::NoMatch }
    } else { OperandMatch::NoMatch }
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

fn test_size_predicate<F>(op: &Option<Operand>, predicate: F, size: OperandSize) -> OperandMatch 
    where F: Fn(&Operand) -> bool {
    if let Some(ref o) = *op {
        if predicate(o) { OperandMatch::Match(Some(size)) } else { OperandMatch::NoMatch }
    } else { OperandMatch::NoMatch }
}

fn match_helper(cond: bool, size: OperandSize) -> OperandMatch {
    if cond { OperandMatch::Match(Some(size)) } else { OperandMatch::NoMatch }
}

fn match_helper_none(cond: bool) -> OperandMatch {
    if cond { OperandMatch::Match(None) } else { OperandMatch::NoMatch }
}

fn encode_rm_direct(buffer: &mut InstructionBuffer, reg: Reg) {
    buffer.mod_rm_mod = Some(0b11);
    buffer.mod_rm_rm = Some(reg.get_reg_code());
}

fn encode_rm_direct_no_mod(buffer: &mut InstructionBuffer, reg: Reg) {
    buffer.mod_rm_rm = Some(reg.get_reg_code());
}

fn encode_rm_indirect(buffer: &mut InstructionBuffer, reg: Reg) {
    buffer.mod_rm_mod = Some(0b00); // Indirect operand
    buffer.mod_rm_rm = Some(reg.get_reg_code());
}

fn encode_rm_indirect_displaced(buffer: &mut InstructionBuffer, reg: Reg, disp: u64) {
    if disp <= <u8>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b01); // Indirect operand (8-bit displacement)
        buffer.immediate = Some(ImmediateValue::Displacement8(disp as u8));
    } else if disp <= <u32>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b10); // Indirect operand (32-bit displacement)
        buffer.immediate = Some(ImmediateValue::Displacement32(disp as u32));
    } else { panic!("Displacement too large for 32-bit mode."); }
    buffer.mod_rm_rm = Some(reg.get_reg_code());
}

fn encode_rm_indirect_scaled_indexed(buffer: &mut InstructionBuffer, base: Reg, index: Reg, scale: RegScale) {
    buffer.mod_rm_mod = Some(0b00); // Indirect operand
    buffer.mod_rm_rm = Some(0b100); // SIB
    buffer.sib_base = Some(base.get_reg_code());
    buffer.sib_index = Some(index.get_reg_code());
    buffer.sib_scale = Some(scale.get_sib_code());
}


fn encode_rm_indirect_scaled_indexed_displaced(buffer: &mut InstructionBuffer, base: Reg, index: Reg, scale: RegScale, disp: u64) {
    if disp <= <u8>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b01); // Indirect operand (8-bit displacement)
        buffer.immediate = Some(ImmediateValue::Displacement8(disp as u8));
    } else if disp <= <u32>::max_value() as u64 {
        buffer.mod_rm_mod = Some(0b10); // Indirect operand (32-bit displacement)
        buffer.immediate = Some(ImmediateValue::Displacement32(disp as u32));
    } else { panic!("Displacement too large for 32-bit mode."); }
    buffer.mod_rm_rm = Some(0b100); // SIB
    buffer.sib_base = Some(base.get_reg_code());
    buffer.sib_index = Some(index.get_reg_code());
    buffer.sib_scale = Some(scale.get_sib_code());
}

fn encode_rm_memory(buffer: &mut InstructionBuffer, addr: u64) {
    buffer.mod_rm_mod = Some(0b01); // Indirect operand
    buffer.mod_rm_rm = Some(0b101); // Fixed address
    buffer.immediate = Some(ImmediateValue::Displacement32(addr as u32));
}

fn encode_reg_direct(buffer: &mut InstructionBuffer, reg: Reg) {
    buffer.mod_rm_reg = Some(reg.get_reg_code());
}

fn reg_matches_mode(mode: Mode, reg: Reg, real_reg: Reg, prot_reg: Reg, long_reg: Reg) -> bool {
    match mode {
        Mode::Real => (real_reg == reg),
        Mode::Protected => (prot_reg == reg),
        Mode::Long => (long_reg == reg),
        Mode::SystemManagement => false
    }
}



#[derive(Copy, Clone, Debug)]
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
    Fixed,  // Fixed operand
}

#[derive(Clone, Copy, Debug)]
pub enum OperandType {
    A,      // Two word/dword operands (memory)
    B,      // Byte
    BCD,    // Packed bcd
    BS,     // Byte (sign-extended to operand size)
    BSQ,    // Byte (sign-extended to qword)
    BSS,    // Byte (sign-extended to stack pointer size)
    C,      // Byte/word
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

#[derive(Clone, Copy, Debug)]
pub enum OpSize64Behavior {
    Normal,
    Force64,
    Force64EvexOnly
}
