use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, RegScale};
use ::test::{encode32_helper2};

#[test]
fn encode_direct() {
    // Test each of the general purpose register direct encodings (mod=3)
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0xD1, 0xC0]); // ROL EAX, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::EBX), Operand::Literal8(0x1), &vec![0xD1, 0xC3]); // ROL EBX, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::ECX), Operand::Literal8(0x1), &vec![0xD1, 0xC1]); // ROL ECX, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::EDX), Operand::Literal8(0x1), &vec![0xD1, 0xC2]); // ROL EDX, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::ESI), Operand::Literal8(0x1), &vec![0xD1, 0xC6]); // ROL ESI, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::EDI), Operand::Literal8(0x1), &vec![0xD1, 0xC7]); // ROL EDI, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::ESP), Operand::Literal8(0x1), &vec![0xD1, 0xC4]); // ROL ESP, 0x1
    encode32_helper2(Mnemonic::ROL, Operand::Direct(Reg::EBP), Operand::Literal8(0x1), &vec![0xD1, 0xC5]); // ROL EBP, 0x1
}

#[test]
fn encode_indirect() {
    // Test indirect encodings (mod=0)
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x00, 0x01]); // ADD BYTE PTR [EAX], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EBX, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x03, 0x01]); // ADD BYTE PTR [EBX], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::ECX, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x01, 0x01]); // ADD BYTE PTR [ECX], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EDX, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x02, 0x01]); // ADD BYTE PTR [EDX], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::ESI, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x06, 0x01]); // ADD BYTE PTR [ESI], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EDI, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x07, 0x01]); // ADD BYTE PTR [EDI], 0x1
    // ESP is special because it requires an SIB
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::ESP, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x04, 0x24, 0x01]); // ADD BYTE PTR [ESP], 0x1
    // EBP is skipped here, because it internally requires a displacement
    // Test a direct memory location (in protected mode, this is encoded using the BP register code)
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Memory(0x12345678, Some(OperandSize::Dword), None), &vec![0x03, 0x05, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, DWORD PTR DS:0x12345678
}

#[test]
fn encode_disp8() {
    // Test indirect + 8-bit displacement encodings (mod=1)
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EAX, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x40, 0x12, 0x01]); // ADD BYTE PTR [EAX+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EBX, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x43, 0x12, 0x01]); // ADD BYTE PTR [EBX+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ECX, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x41, 0x12, 0x01]); // ADD BYTE PTR [ECX+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EDX, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x42, 0x12, 0x01]); // ADD BYTE PTR [EDX+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EDI, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x47, 0x12, 0x01]); // ADD BYTE PTR [EDI+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ESI, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x46, 0x12, 0x01]); // ADD BYTE PTR [ESI+0x12], 0x1
    // ESP is special because it requires an SIB
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ESP, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x44, 0x24, 0x12, 0x01]); // ADD BYTE PTR [ESP+0x12], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EBP, 0x12, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x45, 0x12, 0x01]); // ADD BYTE PTR [EBP+0x12], 0x1
    // EBP is special because there's not an indirect encoding internally, so the it should encode
    // as displacement 0.
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EBP, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x45, 0x00, 0x01]); // ADD BYTE PTR [EBP+0x0], 0x1
}

#[test]
fn encode_disp32() {
    // Test indirect + 32-bit displacment encodings (mod=2)
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EAX, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x80, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [EAX+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EBX, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x83, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [EBX+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ECX, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x81, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [ECX+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EDX, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x82, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [EDX+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ESI, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x86, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [ESI+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EDI, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x87, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [EDI+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::ESP, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x84, 0x24, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [ESP+0x12345678], 0x1
    encode32_helper2(Mnemonic::ADD, Operand::IndirectDisplaced(Reg::EBP, 0x12345678, Some(OperandSize::Byte), None), Operand::Literal8(0x1), &vec![0x80, 0x85, 0x78, 0x56, 0x34, 0x12, 0x01]); // ADD BYTE PTR [EBP+0x12345678], 0x1
}

#[test]
fn encode_sib() {
    // Test SIB encoding
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexed(Reg::EBX, Reg::EDX, RegScale::Two, Some(OperandSize::Dword), None), &vec![0x03, 0x04, 0x53]); // ADD EAX, DWORD PTR [EBX+EDX*2]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexed(Reg::EBP, Reg::EBX, RegScale::Four, Some(OperandSize::Dword), None), &vec![0x03, 0x44, 0x9D, 0x00]); // ADD EAX, DWORD PTR [EBP+EBX*4]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::ESP, Some(OperandSize::Dword), None), &vec![0x03, 0x04, 0x24]); // ADD EAX, [ESP]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledDisplaced(Reg::EBP, RegScale::Four, 0x12345678, Some(OperandSize::Dword), None), &vec![0x03, 0x04, 0xAD, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, DWORD PTR [EBP*4+0x12345678]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectDisplaced(Reg::ESP, 0x12, Some(OperandSize::Dword), None), &vec![0x03, 0x44, 0x24, 0x12]); // ADD EAX, DWORD PTR [ESP+0x12]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexedDisplaced(Reg::EBX, Reg::ECX, RegScale::Eight, 0x12, Some(OperandSize::Dword), None), &vec![0x03, 0x44, 0xCB, 0x12]); // ADD EAX, DWORD PTR EAX, [EBX+ECX*8+0x12]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexedDisplaced(Reg::EBX, Reg::ECX, RegScale::Eight, 0x12345678, Some(OperandSize::Dword), None), &vec![0x03, 0x84, 0xCB, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, DWORD PTR EAX, [EBX+ECX*8+0x12345678]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectDisplaced(Reg::EDX, 0x12345678, Some(OperandSize::Dword), None), &vec![0x03, 0x82, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, DWORD PTR [EDX+0x12345678]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectScaledIndexedDisplaced(Reg::EBX, Reg::ESI, RegScale::One, 0x12345678, Some(OperandSize::Dword), None), &vec![0x03, 0x84, 0x33, 0x78, 0x56, 0x34, 0x12]); // ADD DWORD PTR [EBX+ESI+0x12345678]
}
