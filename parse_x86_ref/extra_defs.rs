// VADDPD
InstructionDefinition { 
    mnemonic: Mnemonic::VADDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VADDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VADDPS
InstructionDefinition { 
    mnemonic: Mnemonic::VADDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VADDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VADDSD
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSD,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSD,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VADDSS
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSS,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::SS,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSS,
    is_two_byte_opcode: true,
    primary_opcode: 0x58,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::SS,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VADDSUBPD
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSUBPD,
    is_two_byte_opcode: true,
    primary_opcode: 0xD0,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VADDSUBPS
InstructionDefinition { 
    mnemonic: Mnemonic::VADDSUBPS,
    is_two_byte_opcode: true,
    primary_opcode: 0xD0,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VANDPD
InstructionDefinition { 
    mnemonic: Mnemonic::VANDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x54,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VANDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x54,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VANDPS
InstructionDefinition { 
    mnemonic: Mnemonic::VANDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x54,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VANDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x54,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VANDNPD
InstructionDefinition { 
    mnemonic: Mnemonic::VANDNPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x55,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VANDNPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x55,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VANDNPS
InstructionDefinition { 
    mnemonic: Mnemonic::VANDNPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x55,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VANDNPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x55,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VBLENDPD
InstructionDefinition { 
    mnemonic: Mnemonic::VBLENDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x0D),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VBLENDPS
InstructionDefinition { 
    mnemonic: Mnemonic::VBLENDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x0C),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VBLENDVPD
InstructionDefinition { 
    mnemonic: Mnemonic::VBLENDVPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x4B),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXImm8,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VBLENDVPS
InstructionDefinition { 
    mnemonic: Mnemonic::VBLENDVPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x4A),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXImm8,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCMPPD
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPPD,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPPD,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskedMaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCMPPS
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPPS,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPPS,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskedMaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCMPSD
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPSD,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPSD,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskedMaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::SD,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCMPSS
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPSS,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::SS,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCMPSS,
    is_two_byte_opcode: true,
    primary_opcode: 0xC2,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskedMaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::SS,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
     can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCOMISD
InstructionDefinition { 
    mnemonic: Mnemonic::VCOMISD,
    is_two_byte_opcode: true,
    primary_opcode: 0x2F,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::V,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::W,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCOMISD,
    is_two_byte_opcode: true,
    primary_opcode: 0x2F,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::V,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::W,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCOMISS
InstructionDefinition { 
    mnemonic: Mnemonic::VCOMISS,
    is_two_byte_opcode: true,
    primary_opcode: 0x2F,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::V,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::W,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCOMISS,
    is_two_byte_opcode: true,
    primary_opcode: 0x2F,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::V,
        operand_type: OperandType::SD,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::W,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTDQ2PD
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To2),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::XMMorMemOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To4),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::XMMorMemOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To8),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::ZMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::YMMorMemOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTDQ2PS
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTDQ2PS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTPD2DQ
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To2),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::XMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To4),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::YMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To8),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::ZMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCVTPD2PS
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2PS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2PS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::XMMorYMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPD2PS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::ZMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCVTPS2DQ
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTPS2PD
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To2),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To4),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::XMMorMemOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTPS2PD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: Some(BroadcastMode::Broadcast1To8),
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::ZMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::YMMorMemOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTSD2SI
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSD2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSD2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTSD2SS
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSD2SS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSD2SS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCVTSI2SD
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSI2SD,
    is_two_byte_opcode: true,
    primary_opcode: 0x2A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSI2SD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VCVTSI2SS
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSI2SS,
    is_two_byte_opcode: true,
    primary_opcode: 0x2A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSI2SS,
    is_two_byte_opcode: true,
    primary_opcode: 0x2A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTSS2SD
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSS2SD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSS2SD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5A,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTSS2SI
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSS2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTSS2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VCVTTPD2DQ
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::XMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::YMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPD2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0xE6,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::ZMMorMemOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// CVTTPS2DQ
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPS2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTPS2DQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x5B,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// CVTTSD2SI
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTSD2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTSD2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// CVTTSS2SI
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTSS2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VCVTTSS2SI,
    is_two_byte_opcode: true,
    primary_opcode: 0x2C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: true,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VDIVPD
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst64Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VDIVPS
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::AVX,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemBcst32Rm,
        operand_type: OperandType::AVX,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VDIVSD
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVSD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVSD,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// VDIVSS
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVSS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VDIVSS,
    is_two_byte_opcode: true,
    primary_opcode: 0x5E,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem32,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// DPPD
InstructionDefinition { 
    mnemonic: Mnemonic::DPPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x41),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VDPPD
InstructionDefinition { 
    mnemonic: Mnemonic::VDPPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x41),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// DPPS
InstructionDefinition { 
    mnemonic: Mnemonic::DPPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x40),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VDPPS
InstructionDefinition { 
    mnemonic: Mnemonic::VDPPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x40),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VDPPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x40),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VHADDPD
InstructionDefinition { 
    mnemonic: Mnemonic::VHADDPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x7C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VHADDPS
InstructionDefinition { 
    mnemonic: Mnemonic::VHADDPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x7C,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VHSUBPD
InstructionDefinition { 
    mnemonic: Mnemonic::VHSUBPD,
    is_two_byte_opcode: true,
    primary_opcode: 0x7D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VHSUBPS
InstructionDefinition { 
    mnemonic: Mnemonic::VHSUBPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x7D,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// VINSERTPS
InstructionDefinition { 
    mnemonic: Mnemonic::VINSERTPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x21),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorYMMorMem32,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// AAD
InstructionDefinition { 
    mnemonic: Mnemonic::AAD,
    is_two_byte_opcode: false,
    primary_opcode: 0xD5,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AAM
InstructionDefinition { 
    mnemonic: Mnemonic::AAM,
    is_two_byte_opcode: false,
    primary_opcode: 0xD4,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// ADCX
InstructionDefinition { 
    mnemonic: Mnemonic::ADCX,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF6),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// ADOX
InstructionDefinition { 
    mnemonic: Mnemonic::ADOX,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF6),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESDEC
InstructionDefinition { 
    mnemonic: Mnemonic::AESDEC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDE),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESDEC
InstructionDefinition { 
    mnemonic: Mnemonic::VAESDEC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDE),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESDECLAST
InstructionDefinition { 
    mnemonic: Mnemonic::AESDECLAST,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDF),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESDECLAST
InstructionDefinition { 
    mnemonic: Mnemonic::VAESDECLAST,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDF),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESENC
InstructionDefinition { 
    mnemonic: Mnemonic::AESENC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDC),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESENC
InstructionDefinition { 
    mnemonic: Mnemonic::VAESENC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDC),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESENCLAST
InstructionDefinition { 
    mnemonic: Mnemonic::AESENCLAST,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDD),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESENCLAST
InstructionDefinition { 
    mnemonic: Mnemonic::VAESENCLAST,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDD),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESIMC
InstructionDefinition { 
    mnemonic: Mnemonic::AESIMC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDB),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESIMC
InstructionDefinition { 
    mnemonic: Mnemonic::VAESIMC,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xDB),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// AESKEYGENASSIST
InstructionDefinition { 
    mnemonic: Mnemonic::AESKEYGENASSIST,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0xDF),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VAESKEYGENASSIST
InstructionDefinition { 
    mnemonic: Mnemonic::VAESKEYGENASSIST,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0xDF),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// ANDN
InstructionDefinition { 
    mnemonic: Mnemonic::ANDN,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF2),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BEXTR
InstructionDefinition { 
    mnemonic: Mnemonic::BEXTR,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF7),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BLSI
InstructionDefinition { 
    mnemonic: Mnemonic::BLSI,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF3),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(3), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BLSMSK
InstructionDefinition { 
    mnemonic: Mnemonic::BLSMSK,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF3),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(2), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BLSR
InstructionDefinition { 
    mnemonic: Mnemonic::BLSR,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF3),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(1), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDCL
InstructionDefinition { 
    mnemonic: Mnemonic::BNDCL,
    is_two_byte_opcode: true,
    primary_opcode: 0x1A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDCU
InstructionDefinition { 
    mnemonic: Mnemonic::BNDCU,
    is_two_byte_opcode: true,
    primary_opcode: 0x1A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDCN
InstructionDefinition { 
    mnemonic: Mnemonic::BNDCN,
    is_two_byte_opcode: true,
    primary_opcode: 0x1B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDLDX
InstructionDefinition { 
    mnemonic: Mnemonic::BNDLDX,
    is_two_byte_opcode: true,
    primary_opcode: 0x1A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::UnsizedMemory,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDMK
InstructionDefinition { 
    mnemonic: Mnemonic::BNDMK,
    is_two_byte_opcode: true,
    primary_opcode: 0x1B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::H,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDMOV
InstructionDefinition { 
    mnemonic: Mnemonic::BNDMOV,
    is_two_byte_opcode: true,
    primary_opcode: 0x1A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundMemRm,
        operand_type: OperandType::BoundOrMem,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::BNDMOV,
    is_two_byte_opcode: true,
    primary_opcode: 0x1B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::BoundOrMem,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BNDSTX
InstructionDefinition { 
    mnemonic: Mnemonic::BNDSTX,
    is_two_byte_opcode: true,
    primary_opcode: 0x1B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::UnsizedMemory,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::BoundReg,
        operand_type: OperandType::Bound,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// BZHI
InstructionDefinition { 
    mnemonic: Mnemonic::BZHI,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF5),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// CLAC
InstructionDefinition { 
    mnemonic: Mnemonic::CLAC,
    is_two_byte_opcode: true,
    primary_opcode: 0x01,
    secondary_opcode: Some(0xCA),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: None,
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// CLFLUSHOPT
InstructionDefinition { 
    mnemonic: Mnemonic::CLFLUSHOPT,
    is_two_byte_opcode: true,
    primary_opcode: 0xAE,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(7), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// CLWB
InstructionDefinition { 
    mnemonic: Mnemonic::CLWB,
    is_two_byte_opcode: true,
    primary_opcode: 0xAE,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(6), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// INVPCID
InstructionDefinition { 
    mnemonic: Mnemonic::INVPCID,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0x82),
    proc_start: None, 
    mode: Mode::Real,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// KADD
InstructionDefinition { 
    mnemonic: Mnemonic::KADDB,
    is_two_byte_opcode: true,
    primary_opcode: 0x4A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KADDW,
    is_two_byte_opcode: true,
    primary_opcode: 0x4A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KADDD,
    is_two_byte_opcode: true,
    primary_opcode: 0x4A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KADDQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x4A,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KAND
InstructionDefinition { 
    mnemonic: Mnemonic::KANDB,
    is_two_byte_opcode: true,
    primary_opcode: 0x41,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDW,
    is_two_byte_opcode: true,
    primary_opcode: 0x41,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDD,
    is_two_byte_opcode: true,
    primary_opcode: 0x41,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x41,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KANDN
InstructionDefinition { 
    mnemonic: Mnemonic::KANDNB,
    is_two_byte_opcode: true,
    primary_opcode: 0x42,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDNW,
    is_two_byte_opcode: true,
    primary_opcode: 0x42,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDND,
    is_two_byte_opcode: true,
    primary_opcode: 0x42,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KANDNQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x42,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KNOTN
InstructionDefinition { 
    mnemonic: Mnemonic::KNOTB,
    is_two_byte_opcode: true,
    primary_opcode: 0x44,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KNOTW,
    is_two_byte_opcode: true,
    primary_opcode: 0x44,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KNOTD,
    is_two_byte_opcode: true,
    primary_opcode: 0x44,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KNOTQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x44,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KOR
InstructionDefinition { 
    mnemonic: Mnemonic::KORB,
    is_two_byte_opcode: true,
    primary_opcode: 0x45,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORW,
    is_two_byte_opcode: true,
    primary_opcode: 0x45,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORD,
    is_two_byte_opcode: true,
    primary_opcode: 0x45,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x45,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KORTEST
InstructionDefinition { 
    mnemonic: Mnemonic::KORTESTB,
    is_two_byte_opcode: true,
    primary_opcode: 0x98,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORTESTW,
    is_two_byte_opcode: true,
    primary_opcode: 0x98,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORTESTD,
    is_two_byte_opcode: true,
    primary_opcode: 0x98,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KORTESTQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x98,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KSHIFTL
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTLB,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x32),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTLW,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x32),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTLD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x33),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTLQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x33),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KSHIFTR
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTRB,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x30),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTRW,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x30),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTRD,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x31),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KSHIFTRQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x31),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KTEST
InstructionDefinition { 
    mnemonic: Mnemonic::KTESTB,
    is_two_byte_opcode: true,
    primary_opcode: 0x99,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KTESTW,
    is_two_byte_opcode: true,
    primary_opcode: 0x99,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KTESTD,
    is_two_byte_opcode: true,
    primary_opcode: 0x99,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KTESTQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x99,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KXNOR
InstructionDefinition { 
    mnemonic: Mnemonic::KXNORB,
    is_two_byte_opcode: true,
    primary_opcode: 0x46,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXNORW,
    is_two_byte_opcode: true,
    primary_opcode: 0x46,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXNORD,
    is_two_byte_opcode: true,
    primary_opcode: 0x46,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXNORQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x46,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KXOR
InstructionDefinition { 
    mnemonic: Mnemonic::KXORB,
    is_two_byte_opcode: true,
    primary_opcode: 0x47,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXORW,
    is_two_byte_opcode: true,
    primary_opcode: 0x47,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXORD,
    is_two_byte_opcode: true,
    primary_opcode: 0x47,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KXORQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x47,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KMOV
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVB,
    is_two_byte_opcode: true,
    primary_opcode: 0x90,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskMemRm,
        operand_type: OperandType::MaskOrMem8,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVW,
    is_two_byte_opcode: true,
    primary_opcode: 0x90,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskMemRm,
        operand_type: OperandType::MaskOrMem16,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x90,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskMemRm,
        operand_type: OperandType::MaskOrMem32,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x90,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskMemRm,
        operand_type: OperandType::MaskOrMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVB,
    is_two_byte_opcode: true,
    primary_opcode: 0x91,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVW,
    is_two_byte_opcode: true,
    primary_opcode: 0x91,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::W,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x91,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x91,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVB,
    is_two_byte_opcode: true,
    primary_opcode: 0x92,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralRm,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVW,
    is_two_byte_opcode: true,
    primary_opcode: 0x92,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralRm,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x92,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralRm,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x92,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Long,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralRm,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVB,
    is_two_byte_opcode: true,
    primary_opcode: 0x93,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVW,
    is_two_byte_opcode: true,
    primary_opcode: 0x93,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x93,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x93,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Long,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(false),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// KUNPCK
InstructionDefinition { 
    mnemonic: Mnemonic::KUNPCKBW,
    is_two_byte_opcode: true,
    primary_opcode: 0x4B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KUNPCKWD,
    is_two_byte_opcode: true,
    primary_opcode: 0x4B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::KUNPCKDQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x4B,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: Some(true),
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskReg,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskVex,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::MaskRm,
        operand_type: OperandType::MaskReg,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// VLDDQU
InstructionDefinition { 
    mnemonic: Mnemonic::VLDDQU,
    is_two_byte_opcode: true,
    primary_opcode: 0xF0,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::XMMorYMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VLDMXCSR
InstructionDefinition { 
    mnemonic: Mnemonic::VLDMXCSR,
    is_two_byte_opcode: true,
    primary_opcode: 0xAE,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::M,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: None,
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: Some(2), proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// LZCNT
InstructionDefinition { 
    mnemonic: Mnemonic::LZCNT,
    is_two_byte_opcode: true,
    primary_opcode: 0xBD,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::VQP,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::VQP,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VEXTRACTPS
InstructionDefinition { 
    mnemonic: Mnemonic::VEXTRACTPS,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x17),
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VMOVD
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x6E,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVD,
    is_two_byte_opcode: true,
    primary_opcode: 0x7E,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
// VMOVQ
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x6E,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x7E,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::D,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Force64
},
// VMOVDDUP
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVDDUP,
    is_two_byte_opcode: true,
    primary_opcode: 0x12,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVDDUP,
    is_two_byte_opcode: true,
    primary_opcode: 0x12,
    secondary_opcode: None,
    proc_start: None, 
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand3: None,
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: false, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVDDUP,
    is_two_byte_opcode: true,
    primary_opcode: 0x12,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMMorMem64,
        fixed_operand: None
    }),
    operand3: None,
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVDDUP,
    is_two_byte_opcode: true,
    primary_opcode: 0x12,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::YMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::YMM,
        fixed_operand: None
    }),
    operand3: None,
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
InstructionDefinition { 
    mnemonic: Mnemonic::VMOVDDUP,
    is_two_byte_opcode: true,
    primary_opcode: 0x12,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: true,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMaskedReg,
        operand_type: OperandType::ZMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::ZMM,
        fixed_operand: None
    }),
    operand3: None,
     can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, secondary_opcode: None, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Force64
},
// MULX
InstructionDefinition { 
    mnemonic: Mnemonic::MULX,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF6),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// PALIGNR
InstructionDefinition { 
    mnemonic: Mnemonic::PALIGNR,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x0F),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::P,
        operand_type: OperandType::Q,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::Q,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
InstructionDefinition { 
    mnemonic: Mnemonic::PALIGNR,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x0F),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: true,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// PAND
InstructionDefinition { 
    mnemonic: Mnemonic::PAND,
    is_two_byte_opcode: true,
    primary_opcode: 0xDB,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::P,
        operand_type: OperandType::Q,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::Q,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand3: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal },
// PANDN
InstructionDefinition { 
    mnemonic: Mnemonic::PANDN,
    is_two_byte_opcode: true,
    primary_opcode: 0xDF,
    secondary_opcode: None,
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::P,
        operand_type: OperandType::Q,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::Q,
        operand_type: OperandType::Q,
        fixed_operand: None
    }),
    operand3: None,
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal
},
// PCLMULQDQ
InstructionDefinition { 
    mnemonic: Mnemonic::PCLMULQDQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x44),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, operand4: None, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal },
// VPCLMULQDQ
InstructionDefinition { 
    mnemonic: Mnemonic::VPCLMULQDQ,
    is_two_byte_opcode: true,
    primary_opcode: 0x3A,
    secondary_opcode: Some(0x44),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: true,
    force_evex: false,
    force_vex: false,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXReg,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXVex,
        operand_type: OperandType::XMM,
        fixed_operand: None,
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::AVXMemRm,
        operand_type: OperandType::XMM,
        fixed_operand: None
    }),
    operand4: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::I,
        operand_type: OperandType::B,
        fixed_operand: None
    }),
    can_lock: false, fixed_prefix: None, flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal },
// PDEP
InstructionDefinition { 
    mnemonic: Mnemonic::PDEP,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF5),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None,
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF2), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal },
// PEXT
InstructionDefinition { 
    mnemonic: Mnemonic::PEXT,
    is_two_byte_opcode: true,
    primary_opcode: 0x38,
    secondary_opcode: Some(0xF5),
    proc_start: Some(ProcessorLevel::Corei7),
    mode: Mode::Protected,
    force_op_size_prefix: false,
    force_evex: false,
    force_vex: true,
    vector_size: None,
    allow_rounding_mode: false,
    allow_sae: false,
    allowed_broadcast: None,
    operand1: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::G,
        operand_type: OperandType::DQP,
        fixed_operand: None,
    }),
    operand2: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::GeneralVex,
        operand_type: OperandType::DQP,
        fixed_operand: None,
    }),
    operand3: Some(OperandDescription {
        addressing_mode: OperandAddressingMode::E,
        operand_type: OperandType::DQP,
        fixed_operand: None
    }),
    operand4: None,
    can_lock: false, fixed_prefix: Some(0xF3), flags: InstructionDefinitionFlags { mem_format: None, tttn: None }, force_addr_size_prefix: false, has_destination: true, opcode_ext: None, proc_end: None, ring_level: RingLevel::Ring3, valid_in_long_mode: true, op_size_64_behavior: OpSize64Behavior::Normal },
