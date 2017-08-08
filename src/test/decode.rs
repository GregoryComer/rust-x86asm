use ::*;
use ::test::*;

#[test]
// Test decoding of the operand size prefix.
fn decode_operand_size_prefix() {
    decode_helper(&vec![0x81, 0xC1, 0x34, 0x12, 0x00, 0x00], Mode::Protected,
        &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::ECX), Operand::Literal32(0x1234))); // ADD ECX, 0x1234
    decode_helper(&vec![0x66, 0x81, 0xC1, 0x34, 0x12], Mode::Protected,
        &Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::CX), Operand::Literal16(0x1234))); // ADD CX, 0x1234
}

#[test]
// Test decoding of the address size prefix.
fn decode_address_size_prefix() {
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
