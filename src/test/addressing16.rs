use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, RegScale};
use ::test::{encode16_helper2};

#[test]
fn encode_direct() {
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::AX), Operand::Literal8(0x1), &vec![0xD1, 0xC0]); // ROL EAX, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::BX), Operand::Literal8(0x1), &vec![0xD1, 0xC3]); // ROL EBX, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::CX), Operand::Literal8(0x1), &vec![0xD1, 0xC1]); // ROL ECX, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::DX), Operand::Literal8(0x1), &vec![0xD1, 0xC2]); // ROL EDX, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::SI), Operand::Literal8(0x1), &vec![0xD1, 0xC6]); // ROL ESI, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::DI), Operand::Literal8(0x1), &vec![0xD1, 0xC7]); // ROL EDI, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::SP), Operand::Literal8(0x1), &vec![0xD1, 0xC4]); // ROL ESP, 0x1
    encode16_helper2(Mnemonic::ROL, Operand::Direct(Reg::BP), Operand::Literal8(0x1), &vec![0xD1, 0xC5]); // ROL EBP, 0x1
}

#[test]
fn encode_indirect() {
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexed(Reg::BX, Reg::SI, RegScale::One, Some(OperandSize::Word), None), &vec![0x03, 0x00]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexed(Reg::BX, Reg::DI, RegScale::One, Some(OperandSize::Word), None), &vec![0x03, 0x01]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexed(Reg::BP, Reg::SI, RegScale::One, Some(OperandSize::Word), None), &vec![0x03, 0x02]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexed(Reg::BP, Reg::DI, RegScale::One, Some(OperandSize::Word), None), &vec![0x03, 0x03]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Indirect(Reg::SI, Some(OperandSize::Word), None), &vec![0x03, 0x04]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Indirect(Reg::DI, Some(OperandSize::Word), None), &vec![0x03, 0x05]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Memory(0x05, Some(OperandSize::Word), None), &vec![0x03, 0x06, 0x05, 0x00]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Indirect(Reg::BX, Some(OperandSize::Word), None), &vec![0x03, 0x07]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Indirect(Reg::BP, Some(OperandSize::Word), None), &vec![0x03, 0x46, 0x00]);
}

#[test]
fn encode_indirect_disp8() {
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::SI, RegScale::One, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x40, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::DI, RegScale::One, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x41, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::SI, RegScale::One, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x42, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::DI, RegScale::One, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x43, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::SI, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x44, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::DI, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x45, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::BP, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x46, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::BX, 0x12, Some(OperandSize::Word), None), &vec![0x03, 0x47, 0x12]);
}

#[test]
fn encode_indirect_disp16() {
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::SI, RegScale::One, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x80, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BX, Reg::DI, RegScale::One, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x81, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::SI, RegScale::One, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x82, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexedDisplaced(Reg::BP, Reg::DI, RegScale::One, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x83, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::SI, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x84, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::DI, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x85, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::BP, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x86, 0x34, 0x12]);
    encode16_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::IndirectDisplaced(Reg::BX, 0x1234, Some(OperandSize::Word), None), &vec![0x03, 0x87, 0x34, 0x12]);
}
