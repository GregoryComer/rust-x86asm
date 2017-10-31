use ::*;
use ::test::*;

#[test]
fn extended_regs() {
    decode_helper(&vec![0x4C, 0x01, 0xC0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::R8)));
    decode_helper(&vec![0x40, 0x00, 0xE0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SPL)));
    decode_helper(&vec![0x40, 0x00, 0xE8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::BPL)));
    decode_helper(&vec![0x40, 0x00, 0xF0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SIL)));
    decode_helper(&vec![0x40, 0x00, 0xF8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::DIL)));
}

// * * * * * * * * * * * * * * * * * * * LEGACY TESTS * * * * * * * * * * * * * * * * * * * *
// The tests below correspond to the legacy instruction encoding format but have been left here
// for completeness.

#[test]
// Test decoding of the operand size prefix.
fn operand_size_prefix() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0x08], Mode::Protected, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1),
        Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Xmmword), None))); // VADDPD XMM1, XMM2, [EAX]
}

#[test]
// Test decoding of the address size prefix.
fn address_size_prefix() {
    // Real mode
    decode_helper(&vec![0x8B, 0x0F], Mode::Real, // MOV CX, [BX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::CX), Operand::Indirect(Reg::BX, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x67, 0x8B, 0x0B], Mode::Real, // MOV CX, [EBX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::CX), Operand::Indirect(Reg::EBX, Some(OperandSize::Word), None)));

    // Protected mode
    decode_helper(&vec![0x8B, 0x0B], Mode::Protected, // MOV ECX, [EBX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::ECX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x67, 0x8B, 0x0F], Mode::Protected, // MOV ECX, [BX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::ECX), Operand::Indirect(Reg::BX, Some(OperandSize::Dword), None)));

    // Long mode
    decode_helper(&vec![0x48, 0x8B, 0x0B], Mode::Long, // MOV RCX, QWORD PTR [RBX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RCX), Operand::Indirect(Reg::RBX, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x67, 0x48, 0x8B, 0x0B], Mode::Long, // MOV RCX, QWORD PTR [EBX]
        &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RCX), Operand::Indirect(Reg::EBX, Some(OperandSize::Qword), None)));
}

#[test]
// Test decoding of real mode Mod/RM mode 0 encodings (no displacement)
fn mod_rm_real_mod_0() {
    decode_helper(&vec![0x8b, 0x00], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+SI]
        Operand::IndirectScaledIndexed(Reg::BX, Reg::SI, RegScale::One, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x01], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+DI]
        Operand::IndirectScaledIndexed(Reg::BX, Reg::DI, RegScale::One, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x02], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+SI]
        Operand::IndirectScaledIndexed(Reg::BP, Reg::SI, RegScale::One, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x03], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+DI]
        Operand::IndirectScaledIndexed(Reg::BP, Reg::DI, RegScale::One, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x04], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [SI]
        Operand::Indirect(Reg::SI, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x05], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [DI]
        Operand::Indirect(Reg::DI, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x06, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [0x1234]
        Operand::Memory(0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x07], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX]
        Operand::Indirect(Reg::BX, Some(OperandSize::Word), None)));
}

#[test]
// Test decoding of real mode Mod/RM mode 1 encodings (8-bit displacement)
fn mod_rm_real_mod_1() {
    decode_helper(&vec![0x8b, 0x40, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+SI+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::SI, RegScale::One, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x41, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+DI+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::DI, RegScale::One, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x42, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+SI+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::SI, RegScale::One, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x43, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+DI+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::DI, RegScale::One, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x44, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [SI+0x12]
        Operand::IndirectDisplaced(Reg::SI, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x45, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [DI+0x12]
        Operand::IndirectDisplaced(Reg::DI, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x46, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+0x12]
        Operand::IndirectDisplaced(Reg::BP, 0x12, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x47, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+0x12]
        Operand::IndirectDisplaced(Reg::BX, 0x12, Some(OperandSize::Word), None)));
}

#[test]
// Test decoding of real mode Mod/RM mode 2 encodings (16-bit displacement)
fn mod_rm_real_mod_2() {
    decode_helper(&vec![0x8b, 0x80, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+SI+0x1234]
        Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::SI, RegScale::One, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x81, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+DI+0x1234]
        Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::DI, RegScale::One, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x82, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+SI+0x1234]
        Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::SI, RegScale::One, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x83, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+DI+0x1234]
        Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::DI, RegScale::One, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x84, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [SI+0x1234]
        Operand::IndirectDisplaced(Reg::SI, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x85, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [DI+0x1234]
        Operand::IndirectDisplaced(Reg::DI, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x86, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BP+0x1234]
        Operand::IndirectDisplaced(Reg::BP, 0x1234, Some(OperandSize::Word), None)));
    decode_helper(&vec![0x8b, 0x87, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), // MOV AX, [BX+0x1234]
        Operand::IndirectDisplaced(Reg::BX, 0x1234, Some(OperandSize::Word), None)));
}

#[test]
// Test decoding of real mode Mod/RM mode 3 encodings (direct register)
fn mod_rm_real_mod_3() {
    decode_helper(&vec![0x01, 0xC0], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::AX))); // ADD AX, AX
    decode_helper(&vec![0x01, 0xD8], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::BX))); // ADD BX, AX
    decode_helper(&vec![0x01, 0xC8], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::CX))); // ADD CX, AX
    decode_helper(&vec![0x01, 0xD0], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::DX))); // ADD DX, AX
    decode_helper(&vec![0x01, 0xE0], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::SP))); // ADD SP, AX
    decode_helper(&vec![0x01, 0xE8], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::BP))); // ADD BP, AX
    decode_helper(&vec![0x01, 0xF0], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::SI))); // ADD SI, AX
    decode_helper(&vec![0x01, 0xF8], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::DI))); // ADD DI, AX
}

#[test]
// Test decoding of protected mode Mod/RM mode 0 encodings (no displacement)
fn mod_rm_protected_mod_0() {
    decode_helper(&vec![0x03, 0x00], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EAX]
        Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x01], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ECX]
        Operand::Indirect(Reg::ECX, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x02], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDX]
        Operand::Indirect(Reg::EDX, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x03], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EBX]
        Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x04, 0x24], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESP]
        Operand::Indirect(Reg::ESP, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x05, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, DS:0x12345678
        Operand::Memory(0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x06], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESI]
        Operand::Indirect(Reg::ESI, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x07], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDI]
        Operand::Indirect(Reg::EDI, Some(OperandSize::Dword), None)));
}

#[test]
// Test decoding of protected mode Mod/RM mode 1 encodings (8-bit displacement)
fn mod_rm_protected_mod_1() {
    decode_helper(&vec![0x03, 0x40, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EAX+0x12]
        Operand::IndirectDisplaced(Reg::EAX, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x41, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ECX+0x12]
        Operand::IndirectDisplaced(Reg::ECX, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x42, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDX+0x12]
        Operand::IndirectDisplaced(Reg::EDX, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x43, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EBX+0x12]
        Operand::IndirectDisplaced(Reg::EBX, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x44, 0x24, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESP+0x12]
        Operand::IndirectDisplaced(Reg::ESP, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x45, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EBP+0x12]
        Operand::IndirectDisplaced(Reg::EBP, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x46, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESI+0x12]
        Operand::IndirectDisplaced(Reg::ESI, 0x12, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x47, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDI+0x12]
        Operand::IndirectDisplaced(Reg::EDI, 0x12, Some(OperandSize::Dword), None)));
}

#[test]
// Test decoding of protected mode Mod/RM mode 2 encodings (32-bit displacement)
fn mod_rm_protected_mod_2() {
    decode_helper(&vec![0x03, 0x80, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EAX+0x12345678]
        Operand::IndirectDisplaced(Reg::EAX, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x81, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ECX+0x12345678]
        Operand::IndirectDisplaced(Reg::ECX, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x82, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDX+0x12345678]
        Operand::IndirectDisplaced(Reg::EDX, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x83, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EBX+0x12345678]
        Operand::IndirectDisplaced(Reg::EBX, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x84, 0x24, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESP+0x12345678]
        Operand::IndirectDisplaced(Reg::ESP, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x85, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EBP+0x12345678]
        Operand::IndirectDisplaced(Reg::EBP, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x86, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ESI+0x12345678]
        Operand::IndirectDisplaced(Reg::ESI, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x87, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [EDI+0x12345678]
        Operand::IndirectDisplaced(Reg::EDI, 0x12345678, Some(OperandSize::Dword), None)));
}

#[test]
// Test decoding of protected mode Mod/RM mode 3 encodings (direct register)
fn mod_rm_protected_mod_3() {
    decode_helper(&vec![0x01, 0xC0], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EAX))); // ADD EAX, EAX
    decode_helper(&vec![0x01, 0xC8], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::ECX))); // ADD EAX, ECX
    decode_helper(&vec![0x01, 0xD0], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EDX))); // ADD EAX, EDX
    decode_helper(&vec![0x01, 0xD8], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX))); // ADD EAX, EBX
    decode_helper(&vec![0x01, 0xE0], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::ESP))); // ADD EAX, ESP
    decode_helper(&vec![0x01, 0xE8], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBP))); // ADD EAX, EBP
    decode_helper(&vec![0x01, 0xF0], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::ESI))); // ADD EAX, ESI
    decode_helper(&vec![0x01, 0xF8], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EDI))); // ADD EAX, EDI
}

#[test]
// Test decoding of protected mode SIB mode 0 encodings (no displacement)
fn sib_protected_mod_0() {
    decode_helper(&vec![0x03, 0x04, 0x58], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexed( // ADD EAX, [EAX+EBX*2]
        Reg::EAX, Reg::EBX, RegScale::Two, Some(OperandSize::Dword), None))); 
    decode_helper(&vec![0x03, 0x04, 0x8D, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD EAX, [ECX*4+0x12345678]
        Operand::IndirectScaledDisplaced(Reg::ECX, RegScale::Four, 0x12345678, Some(OperandSize::Dword), None)));
    decode_helper(&vec![0x03, 0x04, 0x24], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::ESP, // ADD EAX, [ESP]
        Some(OperandSize::Dword), None))); 
}

#[test]
// Test decoding of protected mode SIB mode 1 encodings (8-bit displacement)
fn sib_protected_mod_1() {
    decode_helper(&vec![0x03, 0x44, 0x58, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [EAX+EBX*2+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::EAX, Reg::EBX, RegScale::Two, 0x12, Some(OperandSize::Dword), None))); 
    decode_helper(&vec![0x03, 0x41, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [ECX+0x12]
        Operand::IndirectDisplaced(Reg::ECX, 0x12, Some(OperandSize::Dword), None))); 
    decode_helper(&vec![0x03, 0x44, 0xFE, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [ESI+EDI*8+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::ESI, Reg::EDI, RegScale::Eight, 0x12, Some(OperandSize::Dword), None))); 
}

#[test]
// Test decoding of protected mode SIB mode 2 encodings (32-bit displacement)
fn sib_protected_mod_2() {
    decode_helper(&vec![0x03, 0x84, 0x58, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [EAX+EBX*2+0x12345678]
        Operand::IndirectScaledIndexedDisplaced(Reg::EAX, Reg::EBX, RegScale::Two, 0x12345678, Some(OperandSize::Dword), None))); 
    decode_helper(&vec![0x03, 0x81, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [ECX+0x12345678]
        Operand::IndirectDisplaced(Reg::ECX, 0x12345678, Some(OperandSize::Dword), None))); 
    decode_helper(&vec![0x03, 0x84, 0xFE, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), // ADD [ESI+EDI*8+0x12345678]
        Operand::IndirectScaledIndexedDisplaced(Reg::ESI, Reg::EDI, RegScale::Eight, 0x12345678, Some(OperandSize::Dword), None))); 
}

#[test]
// Test decoding of long mode Mod/RM mode 0 encodings (no displacement)
fn mod_rm_long_mod_0() {
    decode_helper(&vec![0x48, 0x03, 0x00], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX]
        Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x01], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RCX]
        Operand::Indirect(Reg::RCX, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x02], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDX]
        Operand::Indirect(Reg::RDX, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x03], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RBX]
        Operand::Indirect(Reg::RBX, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x04, 0x24], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSP]
        Operand::Indirect(Reg::RSP, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x05, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RIP+0x12345678]
        Operand::Offset(0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x06], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSI]
        Operand::Indirect(Reg::RSI, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x07], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDI]
        Operand::Indirect(Reg::RDI, Some(OperandSize::Qword), None)));
}

#[test]
// Test decoding of long mode Mod/RM mode 1 encodings (8-bit displacement)
fn mod_rm_long_mod_1() {
    decode_helper(&vec![0x48, 0x03, 0x40, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX+0x12]
        Operand::IndirectDisplaced(Reg::RAX, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x41, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RCX+0x12]
        Operand::IndirectDisplaced(Reg::RCX, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x42, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDX+0x12]
        Operand::IndirectDisplaced(Reg::RDX, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x43, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RBX+0x12]
        Operand::IndirectDisplaced(Reg::RBX, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x44, 0x24, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSP+0x12]
        Operand::IndirectDisplaced(Reg::RSP, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x45, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RBP+0x12]
        Operand::IndirectDisplaced(Reg::RBP, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x46, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSI+0x12]
        Operand::IndirectDisplaced(Reg::RSI, 0x12, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x47, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDI+0x12]
        Operand::IndirectDisplaced(Reg::RDI, 0x12, Some(OperandSize::Qword), None)));
}

#[test]
// Test decoding of long mode Mod/RM mode 2 encodings (32-bit displacement)
fn mod_rm_long_mod_2() {
    decode_helper(&vec![0x48, 0x03, 0x80, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX+0x12345678]
        Operand::IndirectDisplaced(Reg::RAX, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x81, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RCX+0x12345678]
        Operand::IndirectDisplaced(Reg::RCX, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x82, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDX+0x12345678]
        Operand::IndirectDisplaced(Reg::RDX, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x83, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RBX+0x12345678]
        Operand::IndirectDisplaced(Reg::RBX, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x84, 0x24, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSP+0x12345678]
        Operand::IndirectDisplaced(Reg::RSP, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x85, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RBP+0x12345678]
        Operand::IndirectDisplaced(Reg::RBP, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x86, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSI+0x12345678]
        Operand::IndirectDisplaced(Reg::RSI, 0x12345678, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x87, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RDI+0x12345678]
        Operand::IndirectDisplaced(Reg::RDI, 0x12345678, Some(OperandSize::Qword), None)));
}

#[test]
// Test decoding of long mode Mod/RM mode 3 encodings (direct register)
fn mod_rm_long_mod_3() {
    decode_helper(&vec![0x48, 0x01, 0xC0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RAX))); // ADD RAX, RAX
    decode_helper(&vec![0x48, 0x01, 0xC8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RCX))); // ADD RAX, RCX
    decode_helper(&vec![0x48, 0x01, 0xD0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RDX))); // ADD RAX, RDX
    decode_helper(&vec![0x48, 0x01, 0xD8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RBX))); // ADD RAX, RBX
    decode_helper(&vec![0x48, 0x01, 0xE0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RSP))); // ADD RAX, RSP
    decode_helper(&vec![0x48, 0x01, 0xE8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RBP))); // ADD RAX, RBP
    decode_helper(&vec![0x48, 0x01, 0xF0], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RSI))); // ADD RAX, RSI
    decode_helper(&vec![0x48, 0x01, 0xF8], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RDI))); // ADD RAX, RDI
}

#[test]
// Test decoding of long mode SIB mode 0 encodings (no displacement)
fn sib_long_mod_0() {
    decode_helper(&vec![0x48, 0x03, 0x04, 0x58], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX+RBX*2]
        Operand::IndirectScaledIndexed(Reg::RAX, Reg::RBX, RegScale::Two, Some(OperandSize::Qword), None)));
    decode_helper(&vec![0x48, 0x03, 0x04, 0x24], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RSP, // ADD RAX, [RSP]
        Some(OperandSize::Qword), None))); 
}

#[test]
// Test decoding of long mode SIB mode 1 encodings (8-bit displacement)
fn sib_long_mod_1() {
    decode_helper(&vec![0x48, 0x03, 0x44, 0x58, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX+RBX*2+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::RAX, Reg::RBX, RegScale::Two, 0x12, Some(OperandSize::Qword), None))); 
    decode_helper(&vec![0x48, 0x03, 0x41, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RCX+0x12]
        Operand::IndirectDisplaced(Reg::RCX, 0x12, Some(OperandSize::Qword), None))); 
    decode_helper(&vec![0x48, 0x03, 0x44, 0xFE, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSI+RDI*8+0x12]
        Operand::IndirectScaledIndexedDisplaced(Reg::RSI, Reg::RDI, RegScale::Eight, 0x12, Some(OperandSize::Qword), None))); 
}

#[test]
// Test decoding of long mode SIB mode 2 encodings (32-bit displacement)
fn sib_long_mod_2() {
    decode_helper(&vec![0x48, 0x03, 0x84, 0x58, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RAX+RBX*2+0x12345678]
        Operand::IndirectScaledIndexedDisplaced(Reg::RAX, Reg::RBX, RegScale::Two, 0x12345678, Some(OperandSize::Qword), None))); 
    decode_helper(&vec![0x48, 0x03, 0x81, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RCX+0x12345678]
        Operand::IndirectDisplaced(Reg::RCX, 0x12345678, Some(OperandSize::Qword), None))); 
    decode_helper(&vec![0x48, 0x03, 0x84, 0xFE, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), // ADD RAX, [RSI+RDI*8+0x12345678]
        Operand::IndirectScaledIndexedDisplaced(Reg::RSI, Reg::RDI, RegScale::Eight, 0x12345678, Some(OperandSize::Qword), None))); 
}

#[test]
fn addressing_mode_a() {
    decode_helper(&vec![0xEA, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::JMP, Operand::MemoryAndSegment32(0x1234, 0x567890AB))); // JMP 0x1234:567890AB
}

#[test]
fn addressing_mode_c() {
    decode_helper(&vec![0x0F, 0x22, 0xE0], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::CR4), Operand::Direct(Reg::EAX))); // MOV CR4, EAX
}

#[test]
fn addressing_mode_d() {
    decode_helper(&vec![0x0F, 0x23, 0xC0], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::DR0), Operand::Direct(Reg::EAX))); // MOV DR0, EAX
}

#[test]
fn addressing_mode_e() {
    decode_helper(&vec![0x01, 0xD8], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX))); // ADD EAX, EBX
    decode_helper(&vec![0x03, 0x03], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX),
        Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None))); // ADD EAX, [EBX]
}

#[test]
fn addressing_mode_es() {
    //decode_helper(&vec![0xD8, 0xC1], Mode::Protected, &Instruction::new1(Mnemonic::FADD, Operand::Direct(Reg::ST1))); // FADD ST(1)
    decode_helper(&vec![0xD8, 0x01], Mode::Protected, &Instruction::new1(Mnemonic::FADD, Operand::Indirect(Reg::ECX, Some(OperandSize::Dword), None))); // FADD DWORD PTR [ECX]
}

#[test]
fn addressing_mode_est() {
    decode_helper(&vec![0xD8, 0xC1], Mode::Protected, &Instruction::new2(Mnemonic::FADD, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1))); // FADD ST(1)
}

#[test]
fn addressing_mode_g() {
    decode_helper(&vec![0x01, 0xCB], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX))); // ADD EBX, ECX
}

#[test]
fn addressing_mode_h() {
    decode_helper(&vec![0x0F, 0x20, 0xE0], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Direct(Reg::CR4))); // ADD MOV, CR4
}

#[test]
fn addressing_mode_i() {
    decode_helper(&vec![0x04, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Literal8(0x12))); // ADD AL, 0x12
    decode_helper(&vec![0x66, 0x05, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Literal16(0x1234))); // ADD AX, 0x1234
    decode_helper(&vec![0x05, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678))); // ADD EAX, 0x12345678
}

#[test]
fn addressing_mode_j() {
    decode_helper(&vec![0xE9, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::JMP, Operand::Offset(0x12345678, None, None))); // JMP 0x12345678
}

#[test]
fn addressing_mode_m() {
    decode_helper(&vec![0x62, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::BOUND, Operand::Direct(Reg::ECX), Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None))); // BOUND ECX, [EAX]
}

#[test]
fn addressing_mode_n() {
    decode_helper(&vec![0x0F, 0x71, 0xD5, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::PSRLW, Operand::Direct(Reg::MM5), Operand::Literal8(0x12))); // PSRLW MM5, 0x12
}

#[test]
fn addressing_mode_o() {
    decode_helper(&vec![0xA0, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AL), Operand::Offset(0x12345678, Some(OperandSize::Byte), None))); // MOV AL, DS:0x12345678
}

#[test]
fn addressing_mode_p() {
    decode_helper(&vec![0x0F, 0xD4, 0xCA], Mode::Protected, &Instruction::new2(Mnemonic::PADDQ, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2))); // PADDQ MM1, MM2
}

#[test]
fn addressing_mode_q() {
    decode_helper(&vec![0x0F, 0xD4, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::PADDQ, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None))); // PADDQ MM1, [EAX]
}

#[test]
fn addressing_mode_r() {
    decode_helper(&vec![0x0F, 0x20, 0xC9], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::ECX), Operand::Direct(Reg::CR1))); // MOV ECX, CR1
}

#[test]
fn addressing_mode_s() {
    decode_helper(&vec![0x8E, 0xD8], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::DS), Operand::Direct(Reg::AX))); // MOV DS, AX
}

// This instruction form seems to be undocumented (at least in the current Intel reference manual),
// so I'm disabling this test for now.
// #[test]
// fn addressing_mode_t() {
//     decode_helper(&vec![0x0F, 0x26, 0xF0], Mode::Protected, &Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::TR6), Operand::Direct(Reg::EAX))); // MOV TR6, EAX
// }

#[test]
fn addressing_mode_u() {
    decode_helper(&vec![0x66, 0x0F, 0x58, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::ADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // ADDPD XMM1, XMM2
}

#[test]
fn addressing_mode_v() {
    decode_helper(&vec![0x0F, 0x58, 0xDD], Mode::Long, &Instruction::new2(Mnemonic::ADDPS, Operand::Direct(Reg::XMM3), Operand::Direct(Reg::XMM5))); // ADDPS XMM3, XMM5
}

#[test]
fn addressing_mode_w() {
    decode_helper(&vec![0x66, 0x0F, 0x5C, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::SUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // SUBPD XMM1, XMM2
    decode_helper(&vec![0x66, 0x0F, 0x5C, 0x08], Mode::Long, &Instruction::new2(Mnemonic::SUBPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Xmmword), None))); // SUBPD XMM1, [RAX]
}

#[test]
fn addressing_mode_z() {
    decode_helper(&vec![0x41], Mode::Protected, &Instruction::new1(Mnemonic::INC, Operand::Direct(Reg::ECX))); // ECX
}

#[test]
fn addressing_mode_avx_vex() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3))); // VADDPD XMM1, XMM2, XMM3
}

#[test]
fn addressing_mode_avx_mem_rm() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0x08], Mode::Protected, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2),
        Operand::Indirect(Reg::EAX, Some(OperandSize::Xmmword), None))); // VADDPD XMM1, XMM2, [EAX]
}

#[test]
fn addressing_mode_avx_reg() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3))); // VADDPD XMM1, XMM2, XMM3
}

#[test]
fn addressing_mode_avx_rm() {
    decode_helper(&vec![0xC5, 0xD9, 0x58, 0xDD], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM3), Operand::Direct(Reg::XMM4), Operand::Direct(Reg::XMM5))); // VADDPD XMM3, XMM4, XMM5
}

#[test]
fn addressing_mode_masked_reg() {
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x0A, 0x58, 0xCB], Mode::Protected, 
        &Instruction {
            mnemonic: Mnemonic::VADDPD,
            operand1: Some(Operand::Direct(Reg::XMM1)),
            operand2: Some(Operand::Direct(Reg::XMM2)),
            operand3: Some(Operand::Direct(Reg::XMM3)),
            mask: Some(MaskReg::K2),
            merge_mode: Some(MergeMode::Merge),
            .. Default::default()
        }); // VADDPD XMM1 {K2}, XMM2, XMM3
}

#[test]
fn addressing_mode_avx_mem_bcast32_rm() {
    decode_helper(&vec![0x62, 0xF1, 0x6C, 0x18, 0x5E, 0x08], Mode::Protected, 
        &Instruction {
            mnemonic: Mnemonic::VDIVPS,
            operand1: Some(Operand::Direct(Reg::XMM1)),
            operand2: Some(Operand::Direct(Reg::XMM2)),
            operand3: Some(Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None)),
            broadcast: Some(BroadcastMode::Broadcast1To4),
            .. Default::default()
        }); // VDIVPS XMM1, XMM2, [EAX] {1to4}
}

#[test]
fn addressing_mode_avx_mem_bcast64_rm() {
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x18, 0x5E, 0x08], Mode::Protected,
        &Instruction {
            mnemonic: Mnemonic::VDIVPD,
            operand1: Some(Operand::Direct(Reg::XMM1)),
            operand2: Some(Operand::Direct(Reg::XMM2)),
            operand3: Some(Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None)),
            broadcast: Some(BroadcastMode::Broadcast1To2),
            .. Default::default()
        }); // VDIVPD XMM1, XMM2, [EAX] {1to2}
}

#[test]
fn addressing_mode_avx_imm8() {
    decode_helper(&vec![0xC4, 0xE3, 0x69, 0x4B, 0xCB, 0x40], Mode::Long,
        &Instruction {
            mnemonic: Mnemonic::VBLENDVPD, 
            operand1: Some(Operand::Direct(Reg::XMM1)),
            operand2: Some(Operand::Direct(Reg::XMM2)),
            operand3: Some(Operand::Direct(Reg::XMM3)),
            operand4: Some(Operand::Direct(Reg::XMM4)),
            .. Default::default()
        });  // VBLENDVPD XMM1, XMM2, XMM3, XMM4
}

#[test]
fn addressing_mode_avx_dest_mem_rm() {
    decode_helper(&vec![0x62, 0xF1, 0x7C, 0x2B, 0x29, 0x18], Mode::Protected,
        &Instruction {
            mnemonic: Mnemonic::VMOVAPS,
            operand1: Some(Operand::Indirect(Reg::EAX, Some(OperandSize::Ymmword), None)),
            operand2: Some(Operand::Direct(Reg::YMM3)),
            mask: Some(MaskReg::K3),
            merge_mode: Some(MergeMode::Merge),
            .. Default::default()
        }); // VMOVAPS [EAX] {K3}, YMM3
}

#[test]
fn addressing_mode_avx_reg_masked_rm() {
    decode_helper(&vec![0x62, 0xF1, 0xFF, 0x0D, 0x10, 0x10], Mode::Protected,
        &Instruction {
            mnemonic: Mnemonic::VMOVSD,
            operand1: Some(Operand::Direct(Reg::XMM2)),
            operand2: Some(Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None)),
            mask: Some(MaskReg::K5),
            merge_mode: Some(MergeMode::Merge),
            .. Default::default()
        }); // VMOVSD XMM2 {K5}, [EAX]
}

#[test]
fn addressing_mode_masked_mask_reg() {
    decode_helper(&vec![0x62, 0xF1, 0x6D, 0x2B, 0x74, 0xD3], Mode::Long,
        &Instruction {
            mnemonic: Mnemonic::VPCMPEQB, 
            operand1: Some(Operand::Direct(Reg::K2)), 
            operand2: Some(Operand::Direct(Reg::YMM2)),
            operand3: Some(Operand::Direct(Reg::YMM3)),
            mask: Some(MaskReg::K3),
            merge_mode: Some(MergeMode::Merge),
            .. Default::default()
        }); // VPCMPEQB K2 {K3}, YMM2, YMM3
}

#[test]
fn addressing_mode_mask_reg() {
    decode_helper(&vec![0xC5, 0xE5, 0x4A, 0xD4], Mode::Protected, 
        &Instruction {
            mnemonic: Mnemonic::KADDB,
            operand1: Some(Operand::Direct(Reg::K2)),
            operand2: Some(Operand::Direct(Reg::K3)),
            operand3: Some(Operand::Direct(Reg::K4)),
            .. Default::default()
        }); // KADDB K2, K3, K4
}

#[test]
fn addressing_mode_mask_rm() {
    decode_helper(&vec![0xC5, 0xDD, 0x4A, 0xDD], Mode::Long,
        &Instruction {
            mnemonic: Mnemonic::KADDB,
            operand1: Some(Operand::Direct(Reg::K3)),
            operand2: Some(Operand::Direct(Reg::K4)),
            operand3: Some(Operand::Direct(Reg::K5)),
            .. Default::default()
        }); // KADDB K3, K4, K5
}

#[test]
fn addressing_mode_mask_vex() {
    decode_helper(&vec![0xC5, 0xD5, 0x4A, 0xE6], Mode::Long, &Instruction::new3(Mnemonic::KADDB, Operand::Direct(Reg::K4), Operand::Direct(Reg::K5), Operand::Direct(Reg::K6))); // KADDB K4, K5, K6
}

#[test]
fn addressing_mode_mask_mem_rm() {
    decode_helper(&vec![0xC5, 0xF9, 0x90, 0x30], Mode::Protected, &Instruction::new2(Mnemonic::KMOVB, Operand::Direct(Reg::K6), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None))); // KMOVB K6, [EAX]
}

#[test]
fn addressing_mode_general_vex() {
    decode_helper(&vec![0xC4, 0xE2, 0x63, 0xF6, 0xC1], Mode::Protected, &Instruction::new3(Mnemonic::MULX, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX))); // MULX EAX, EBX, ECX
}

#[test]
fn addressing_mode_general_rm() {
    decode_helper(&vec![0xC5, 0xFB, 0x92, 0xC8], Mode::Protected, &Instruction::new2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Direct(Reg::EAX))); // KMOVD K1, EAX
}

#[test]
fn addressing_mode_bound_reg() {
    decode_helper(&vec![0xF3, 0x0F, 0x1A, 0xC8], Mode::Protected, &Instruction::new2(Mnemonic::BNDCL, Operand::Direct(Reg::BND1), Operand::Direct(Reg::EAX))); // BNDCL BND1, EAX
}

#[test]
fn addressing_mode_bound_mem_rm() {
    decode_helper(&vec![0xF3, 0x0F, 0x1A, 0xC8], Mode::Protected, &Instruction::new2(Mnemonic::BNDCL, Operand::Direct(Reg::BND1), Operand::Direct(Reg::EAX))); // BNDCL BND1, BND2
    decode_helper(&vec![0xF3, 0x0F, 0x1A, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::BNDCL, Operand::Direct(Reg::BND1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None))); // BNDCL BND1, [EAX]
}

#[test]
fn operand_type_a() {
    decode_helper(&vec![0x66, 0x62, 0x00], Mode::Protected, &Instruction::new2(Mnemonic::BOUND, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None))); // BOUND AX, [EAX]
    decode_helper(&vec![0x62, 0x00], Mode::Protected, &Instruction::new2(Mnemonic::BOUND, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None))); // BOUND EAX, [EAX]
}

#[test]
fn operand_type_b() {
    decode_helper(&vec![0xC4, 0xE3, 0x79, 0x32, 0xCA, 0x05], Mode::Protected, &Instruction::new3(Mnemonic::KSHIFTLB, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), Operand::Literal8(5))); // KSHIFTLB K1, K2, 5
}

#[test]
fn operand_type_bcd() {
    decode_helper(&vec![0xDF, 0x20], Mode::Protected, &Instruction::new1(Mnemonic::FBLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None))); // FBLD [EAX]
}

#[test]
fn operand_type_bs() {
    decode_helper(&vec![0x6B, 0xC3, 0x12], Mode::Protected, &Instruction::new3(Mnemonic::IMUL, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Literal8(0x12))); // IMUL EAX, EBX, 0x12
}

#[test]
fn operand_type_bss() {
    decode_helper(&vec![0x6A, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::PUSH, Operand::Literal8(0x12))); // PUSH 0x12
}

#[test]
fn operand_type_d() {
    decode_helper(&vec![0x0F, 0x6E, 0xD0], Mode::Protected, &Instruction::new2(Mnemonic::MOVD, Operand::Direct(Reg::MM2), Operand::Direct(Reg::EAX))); // MOVD MM2, EAX
}

#[test]
fn operand_type_di() {
    decode_helper(&vec![0xDA, 0x00], Mode::Protected, &Instruction::new1(Mnemonic::FIADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None))); // FIADD DWORD PTR [EAX}
}

#[test]
fn operand_type_dq() {
    decode_helper(&vec![0x66, 0x0F, 0x38, 0x00, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::PSHUFB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Xmmword), None))); // PSHUFB XMM1, [EAX]
    decode_helper(&vec![0x66, 0x0F, 0x38, 0x00, 0xCA], Mode::Protected, &Instruction::new2(Mnemonic::PSHUFB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // PSHUFB XMM1, [EAX]
}

#[test]
fn operand_type_dqp() {
    decode_helper(&vec![0xF2, 0x48, 0x0F, 0x38, 0xF0, 0xC0], Mode::Long, &Instruction::new2(Mnemonic::CRC32, Operand::Direct(Reg::RAX), Operand::Direct(Reg::AL))); // CRC32 RAX, AL
    decode_helper(&vec![0xF2, 0x0F, 0x38, 0xF0, 0xC0], Mode::Protected, &Instruction::new2(Mnemonic::CRC32, Operand::Direct(Reg::EAX), Operand::Direct(Reg::AL))); // CRC32 EAX, AL
}

#[test]
fn operand_type_dr() {
    decode_helper(&vec![0xDC, 0x00], Mode::Protected, &Instruction::new1(Mnemonic::FADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None))); // FADD QWORD PTR [EAX]
}

#[test]
fn operand_type_ds() {
    decode_helper(&vec![0xE8, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::CALL, Operand::Offset(0x12345678, None, None))); // CALL 0x12345678
}

// I've temporarily disabled this test as the decoding logic will need to lookahead in order to
// distiguish between a standalone FWAIT instruction and an instruction prefixed with 0x9B.
// #[test]
// fn operand_type_e() {
//     decode_helper(&vec![0x9B, 0xD9, 0x30], Mode::Protected, &Instruction::new1(Mnemonic::FSTENV, Operand::Indirect(Reg::EAX, None, None))); // FSTENV [EAX]
// }

#[test]
fn operand_type_er() {
    decode_helper(&vec![0xDB, 0x28], Mode::Protected, &Instruction::new1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None))); // FLD TBYTE PTR [EAX]
}

#[test]
fn operand_type_p() {
    decode_helper(&vec![0x9A, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01], Mode::Protected, &Instruction::new1(Mnemonic::CALL, Operand::MemoryAndSegment32(0x0123, 0x456789AB))); // CALL 0x0123:0x456789AB
    decode_helper(&vec![0x66, 0x9A, 0x67, 0x45, 0x23, 0x01], Mode::Protected, &Instruction::new1(Mnemonic::CALL, Operand::MemoryAndSegment16(0x0123, 0x4567))); // CALL 0x0123:0x4567
    decode_helper(&vec![0x9A, 0x67, 0x45, 0x23, 0x01], Mode::Real, &Instruction::new1(Mnemonic::CALL, Operand::MemoryAndSegment16(0x0123, 0x4567))); // CALL 0x0123:0x4567
}

#[test]
fn operand_type_pi() {
    decode_helper(&vec![0x0F, 0x2A, 0xCA], Mode::Protected, &Instruction::new2(Mnemonic::CVTPI2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::MM2))); // CVTPI2PS XMM1, MM2
}

#[test]
fn operand_type_pd() {
    decode_helper(&vec![0x66, 0x0F, 0x58, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::ADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // ADDPD XMM1, XMM2
    decode_helper(&vec![0x66, 0x0F, 0x58, 0x08], Mode::Long, &Instruction::new2(Mnemonic::ADDPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Xmmword), None))); // ADDPD XMM1, [EAX]1
}

#[test]
fn operand_type_ps() {
    decode_helper(&vec![0x0F, 0x58, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::ADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // ADDPS XMM1, XMM2
    decode_helper(&vec![0x0F, 0x58, 0x08], Mode::Long, &Instruction::new2(Mnemonic::ADDPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Xmmword), None))); // ADDPS XMM1, [EAX]1
}

#[test]
fn operand_type_psq() {
    decode_helper(&vec![0x0F, 0x2C, 0xCA], Mode::Protected, &Instruction::new2(Mnemonic::CVTTPS2PI, Operand::Direct(Reg::MM1), Operand::Direct(Reg::XMM2))); // CVTTPS2PI MM1, XMM2
    decode_helper(&vec![0x0F, 0x2C, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::CVTTPS2PI, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None))); // CVTTPS2PI MM1, [EAX]
}

#[test]
fn operand_type_ptp() {
    decode_helper(&vec![0xFF, 0x10], Mode::Protected, &Instruction::new1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None))); // CALL DWORD PTR [EAX]
    decode_helper(&vec![0xFF, 0x18], Mode::Protected, &Instruction::new1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Fword), None))); // CALL FWORD PTR [EAX]

    // TODO I'm not 100% sure this is correct. It seems to be from the Intel docs, but GCC won't
    // seem to accept this form?
    decode_helper(&vec![0x48, 0xFF, 0x18], Mode::Long, &Instruction::new1(Mnemonic::CALL, Operand::Indirect(Reg::RAX, Some(OperandSize::Tbyte), None))); // CALL TBYTE PTR [EAX]
}

#[test]
fn operand_type_q() {
    decode_helper(&vec![0xFF, 0x20], Mode::Long, &Instruction::new1(Mnemonic::JMP, Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None))); // JMP QWORD PTR [RAX]
}

#[test]
fn operand_type_qp() {
    decode_helper(&vec![0x48, 0xCF], Mode::Long, &Instruction::new0(Mnemonic::IRETQ)); // IRETQ
}

#[test]
fn operand_type_sd() {
    decode_helper(&vec![0xF2, 0x0F, 0x58, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::ADDSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // ADDSD XMM1, XMM2
    decode_helper(&vec![0xF2, 0x0F, 0x58, 0x08], Mode::Long, &Instruction::new2(Mnemonic::ADDSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None))); // ADDSD XMM1, [RAX]
}

#[test]
fn operand_type_ss() {
    decode_helper(&vec![0xF3, 0x0F, 0x58, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::ADDSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // ADDSS XMM1, XMM2
    decode_helper(&vec![0xF3, 0x0F, 0x58, 0x08], Mode::Long, &Instruction::new2(Mnemonic::ADDSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Dword), None))); // ADDSS XMM1, [RAX]
}

#[test]
fn operand_type_v() {
    decode_helper(&vec![0x40], Mode::Real, &Instruction::new1(Mnemonic::INC, Operand::Direct(Reg::AX))); // INC AX
    decode_helper(&vec![0x66, 0x40], Mode::Real, &Instruction::new1(Mnemonic::INC, Operand::Direct(Reg::EAX))); // INC EAX
    decode_helper(&vec![0x66, 0x40], Mode::Protected, &Instruction::new1(Mnemonic::INC, Operand::Direct(Reg::AX))); // INC AX
    decode_helper(&vec![0x40], Mode::Protected, &Instruction::new1(Mnemonic::INC, Operand::Direct(Reg::EAX))); // INC EAX
}

#[test]
fn operand_type_vds() {
    decode_helper(&vec![0x05, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Literal16(0x1234))); // ADD AX, 0x1234
    decode_helper(&vec![0x66, 0x05, 0x78, 0x56, 0x34, 0x12], Mode::Real, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678))); // ADD EAX, 0x12345678
    decode_helper(&vec![0x66, 0x05, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Literal16(0x1234))); // ADD AX, 0x1234
    decode_helper(&vec![0x05, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678))); // ADD EAX, 0x12345678
    decode_helper(&vec![0x05, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678))); // ADD RAX, 0x12345678
    decode_helper(&vec![0x48, 0x05, 0x78, 0x56, 0x34, 0x12], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Literal32(0x12345678))); // ADD RAX, 0x12345678
}

#[test]
fn operand_type_vq() {
    decode_helper(&vec![0x66, 0x50], Mode::Long, &Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::AX))); // PUSH AX
    decode_helper(&vec![0x50], Mode::Long, &Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::RAX))); // PUSH RAX
}

#[test]
fn operand_type_vqp() {
    decode_helper(&vec![0x66, 0x01, 0xC3], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Direct(Reg::AX))); // ADD BX, AX
    decode_helper(&vec![0x01, 0xC3], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Direct(Reg::EAX))); // ADD EBX, EAX
    decode_helper(&vec![0x48, 0x01, 0xC3], Mode::Long, &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RBX), Operand::Direct(Reg::RAX))); // ADD RBX, RAX
}

#[test]
fn operand_type_vs() {
    decode_helper(&vec![0x66, 0x68, 0x34, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::PUSH, Operand::Literal16(0x1234))); // PUSH 0x1234
    decode_helper(&vec![0x68, 0x78, 0x56, 0x34, 0x12], Mode::Protected, &Instruction::new1(Mnemonic::PUSH, Operand::Literal32(0x12345678))); // PUSH 0x12345678
}

#[test]
fn operand_type_w() {
    decode_helper(&vec![0xC8, 0x05, 0x00, 0x06], Mode::Real, &Instruction::new2(Mnemonic::ENTER, Operand::Literal16(0x5), Operand::Literal8(0x06))); // ENTER 5, 6
    decode_helper(&vec![0xC8, 0x05, 0x00, 0x06], Mode::Protected, &Instruction::new2(Mnemonic::ENTER, Operand::Literal16(0x5), Operand::Literal8(0x06))); // ENTER 5, 6
    decode_helper(&vec![0xC8, 0x05, 0x00, 0x06], Mode::Long, &Instruction::new2(Mnemonic::ENTER, Operand::Literal16(0x5), Operand::Literal8(0x06))); // ENTER 5, 6
}

#[test]
fn operand_type_xmm() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3))); // VADDPD XMM1, XMM2, XMM3
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0x08], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2),
        Operand::Indirect(Reg::RAX, Some(OperandSize::Xmmword), None))); // VADDPD XMM1, XMM2, [EAX}
}

#[test]
fn operand_type_ymm() {
    decode_helper(&vec![0xC5, 0xED, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3))); // VADDPD YMM1, YMM2, YMM3
    decode_helper(&vec![0xC5, 0xED, 0x58, 0x08], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2),
        Operand::Indirect(Reg::RAX, Some(OperandSize::Ymmword), None))); // VADDPD YMM1, YMM2, [EAX}
}

#[test]
fn operand_type_zmm() {
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x48, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3))); // VADDPD ZMM1, ZMM2, ZMM3
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x48, 0x58, 0x08], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2),
        Operand::Indirect(Reg::RAX, Some(OperandSize::Zmmword), None))); // VADDPD ZMM1, ZMM2, [EAX}
}

#[test]
fn operand_type_xmm_or_ymm() {
    decode_helper(&vec![0xC5, 0xE9, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3))); // VADDPD XMM1, XMM2, XMM3
    decode_helper(&vec![0xC5, 0xED, 0x58, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3))); // VADDPD YMM1, YMM2, YMM3
}

#[test]
fn operand_type_xmm_or_ymm_or_mem_or_mem64() {
    decode_helper(&vec![0xC5, 0xF9, 0x5A, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // VCVTPD2PS XMM1, XMM2
    decode_helper(&vec![0xC5, 0xF9, 0x5A, 0x08], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Xmmword), None))); // VCVTPD2PS XMM1, XMMWORD PTR [RAX]
    decode_helper(&vec![0x62, 0xF1, 0xFD, 0x18, 0x5A, 0x08], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VCVTPD2PS, 
        operand1: Some(Operand::Direct(Reg::XMM1)),
        operand2: Some(Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None)),
        broadcast: Some(BroadcastMode::Broadcast1To2),
        .. Default::default()
    }); // VCVTPD2PS XMM1, QWORD PTR [RAX] {1to2}
    decode_helper(&vec![0x62, 0xF1, 0xFD, 0x38, 0x5A, 0x08], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VCVTPD2PS,
        operand1: Some(Operand::Direct(Reg::XMM1)),
        operand2: Some(Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None)),
        broadcast: Some(BroadcastMode::Broadcast1To4),
        .. Default::default()
    }); // VCVTPD2PS XMM1, QWORD PTR [RAX] {1to4}
}

#[test]
fn operand_type_xmm_or_mem32() {
    decode_helper(&vec![0xC5, 0xEA, 0x5A, 0xCB], Mode::Long, &Instruction::new3(Mnemonic::VCVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3))); // VCVTSS2SD XMM1, XMM2
    decode_helper(&vec![0xC5, 0xEA, 0x5A, 0x08], Mode::Long, &Instruction::new3(Mnemonic::VCVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2),
        Operand::Indirect(Reg::RAX, Some(OperandSize::Dword), None))); // VCVTSS2SD XMM1, DWORD PTR [RAX]
}

// #[test]
// fn operand_type_xmm_or_mem_or_mem32() {
//     // TODO Can't seem to get GAS to assemble VCVTDQ2PD in the form to test the appropriate form?
//     panic!();
// }
// 
// #[test]
// fn operand_type_xmm_or_mem_or_mem64() {
//     // See xmm_or_mem_or_mem32
//     panic!();
// }

#[test]
fn operand_type_xmm_or_mem64() {
    decode_helper(&vec![0x0F, 0x5A, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::CVTPS2PD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2))); // CVTPS2PD XMM1, XMM2
    decode_helper(&vec![0x0F, 0x5A, 0x08], Mode::Long, &Instruction::new2(Mnemonic::CVTPS2PD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None))); // CVTPS2PD XMM1, QWORD PTR [RAX]
}

#[test]
fn operand_type_ymm_or_mem_or_mem32() {
    decode_helper(&vec![0x62, 0xF1, 0x7C, 0x48, 0x5A, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::YMM2))); // VCVTPS2PD ZMM1, YMM2
    decode_helper(&vec![0x62, 0xF1, 0x7C, 0x48, 0x5A, 0x08], Mode::Long, &Instruction::new2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::ZMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Ymmword), None))); // VCVTPS2PD ZMM1, YMMWORD PTR [RAX]
    decode_helper(&vec![0x62, 0xF1, 0x7C, 0x58, 0x5A, 0x08], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VCVTPS2PD, 
        operand1: Some(Operand::Direct(Reg::ZMM1)),
        operand2: Some(Operand::Indirect(Reg::RAX, Some(OperandSize::Dword), None)),
        broadcast: Some(BroadcastMode::Broadcast1To8),
        .. Default::default()
    }); // VCVTPS2PD ZMM1, DWORD PTR [RAX] {1to8}
}

#[test]
fn operand_type_ymm_or_mem_or_mem64() {
    decode_helper(&vec![0xC5, 0xFF, 0xE6, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::YMM2))); // VCVTPD2DQ XMM1, YMM2
    decode_helper(&vec![0xC5, 0xFF, 0xE6, 0x08], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::RAX, Some(OperandSize::Ymmword), None))); // VCVTPD2DQ XMM1, YMMWORD PTR [RAX]
    decode_helper(&vec![0x62, 0xF1, 0xFF, 0x38, 0xE6, 0x08], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VCVTPD2DQ,
        operand1: Some(Operand::Direct(Reg::XMM1)),
        operand2: Some(Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None)),
        broadcast: Some(BroadcastMode::Broadcast1To4),
        .. Default::default()
    }); // VCVTPD2DQ XMM1, YMMWORD PTR [RAX]
}

#[test]
fn operand_type_zmm_or_mem_or_mem64() {
    decode_helper(&vec![0x62, 0xF1, 0xFD, 0x48, 0x5A, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::ZMM2))); // VCVTPD2PS YMM1, ZMM2
    decode_helper(&vec![0x62, 0xF1, 0xFD, 0x48, 0x5A, 0x08], Mode::Long, &Instruction::new2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::YMM1),
        Operand::Indirect(Reg::RAX, Some(OperandSize::Zmmword), None))); // VCVTPD2PS YMM1, ZMMWORD PTR [RAX]
    decode_helper(&vec![0x62, 0xF1, 0xFD, 0x58, 0x5A, 0x08], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VCVTPD2PS,
        operand1: Some(Operand::Direct(Reg::YMM1)),
        operand2: Some(Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None)),
        broadcast: Some(BroadcastMode::Broadcast1To8),
        .. Default::default()
    }); // VCVTPD2PS YMM1, QWORD PTR [RAX], {1to8}
}

#[test]
fn operand_type_avx() {
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x0A, 0x58, 0xCB], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VADDPD,
        operand1: Some(Operand::Direct(Reg::XMM1)),
        operand2: Some(Operand::Direct(Reg::XMM2)),
        operand3: Some(Operand::Direct(Reg::XMM3)),
        mask: Some(MaskReg::K2),
        merge_mode: Some(MergeMode::Merge),
        .. Default::default()
    }); // VADDPD XMM1 {K2}, XMM2, XMM3
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x2A, 0x58, 0xCB], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VADDPD,
        operand1: Some(Operand::Direct(Reg::YMM1)),
        operand2: Some(Operand::Direct(Reg::YMM2)),
        operand3: Some(Operand::Direct(Reg::YMM3)),
        mask: Some(MaskReg::K2),
        merge_mode: Some(MergeMode::Merge),
        .. Default::default()
    }); // VADDPD YMM1 {K2}, YMM2, YMM3
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x4A, 0x58, 0xCB], Mode::Long, &Instruction {
        mnemonic: Mnemonic::VADDPD,
        operand1: Some(Operand::Direct(Reg::ZMM1)),
        operand2: Some(Operand::Direct(Reg::ZMM2)),
        operand3: Some(Operand::Direct(Reg::ZMM3)),
        mask: Some(MaskReg::K2),
        merge_mode: Some(MergeMode::Merge),
        .. Default::default()
    }); // VADDPD ZMM1 {K2}, ZMM2, ZMM3
}

#[test]
fn operand_type_mask_reg() {
    decode_helper(&vec![0x62, 0xF1, 0xED, 0x28, 0xC2, 0xDB, 0x05], Mode::Long, &Instruction::new4(Mnemonic::VCMPPD, Operand::Direct(Reg::K3), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(5))); // VCMPPD K3, YMM2, YMM3, 5
}

#[test]
fn operand_type_mask_or_mem_8() {
    decode_helper(&vec![0xC5, 0xF9, 0x90, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::KMOVB, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2))); // KMOVB K1, K2
    decode_helper(&vec![0xC5, 0xF9, 0x90, 0x08], Mode::Long, &Instruction::new2(Mnemonic::KMOVB, Operand::Direct(Reg::K1), Operand::Indirect(Reg::RAX, Some(OperandSize::Byte), None))); // KMOVB K1, BYTE PTR [RAX]
}

#[test]
fn operand_type_mask_or_mem_16() {
    decode_helper(&vec![0xC5, 0xF8, 0x90, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::KMOVW, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2))); // KMOVW K1, K2
    decode_helper(&vec![0xC5, 0xF8, 0x90, 0x08], Mode::Long, &Instruction::new2(Mnemonic::KMOVW, Operand::Direct(Reg::K1), Operand::Indirect(Reg::RAX, Some(OperandSize::Word), None))); // KMOVW K1, BYTE PTR [RAX]
}

#[test]
fn operand_type_mask_or_mem_32() {
    decode_helper(&vec![0xC4, 0xE1, 0xF9, 0x90, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2))); // KMOVD K1, K2
    decode_helper(&vec![0xC4, 0xE1, 0xF9, 0x90, 0x08], Mode::Long, &Instruction::new2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Indirect(Reg::RAX, Some(OperandSize::Dword), None))); // KMOVD K1, DWORD PTR [RAX]
}

#[test]
fn operand_type_mask_or_mem_64() {
    decode_helper(&vec![0xC4, 0xE1, 0xF8, 0x90, 0xCA], Mode::Long, &Instruction::new2(Mnemonic::KMOVQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2))); // KMOVQ K1, K2
    decode_helper(&vec![0xC4, 0xE1, 0xF8, 0x90, 0x08], Mode::Long, &Instruction::new2(Mnemonic::KMOVQ, Operand::Direct(Reg::K1), Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None))); // KMOVQ K1, QWORD PTR [RAX]
}

#[test]
fn operand_type_bound() {
    decode_helper(&vec![0xF3, 0x0F, 0x1A, 0xC8], Mode::Protected, &Instruction::new2(Mnemonic::BNDCL, Operand::Direct(Reg::BND1), Operand::Direct(Reg::EAX))); // BNDCL BND1, EAX
}

#[test]
fn operand_type_bound_or_mem() {
    decode_helper(&vec![0x66, 0x0F, 0x1A, 0xCA], Mode::Protected, &Instruction::new2(Mnemonic::BNDMOV, Operand::Direct(Reg::BND1), Operand::Direct(Reg::BND2))); // BNDMOV BND1, BND2
    decode_helper(&vec![0x66, 0x0F, 0x1A, 0x08], Mode::Protected, &Instruction::new2(Mnemonic::BNDMOV, Operand::Direct(Reg::BND1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None))); // BNDMOV BND1, QWORD PTR [EAX]
}

#[test]
fn operand_type_unsized_memory() {
    decode_helper(&vec![0x8D, 0x03], Mode::Protected, &Instruction::new2(Mnemonic::LEA, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Unsized), None))); // LEA EAX, [EBX]
}

#[test]
fn operand_type_fpu_register() {
    decode_helper(&vec![0xD8, 0xC2], Mode::Protected, &Instruction::new2(Mnemonic::FADD, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2))); // FADD ST(2)
}
