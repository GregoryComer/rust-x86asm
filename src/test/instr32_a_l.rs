use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, InstructionFlags, RegScale};
use ::test::{encode32_helper0, encode32_helper1, encode32_helper2, encode32_helper2_flags, encode32_helper3, encode32_helper3_flags, encode32_helper4, test_aliased};

#[test]
fn instr_aaa() {
    encode32_helper0(Mnemonic::AAA, &vec![0x37]); // AAA
}

#[test]
fn instr_aad() {
    encode32_helper0(Mnemonic::AAD, &vec![0xD5, 0x0A]); // AAD
    encode32_helper1(Mnemonic::AAD, Operand::Literal8(0x3), &vec![0xD5, 0x03]); // AAD 3
}

#[test]
fn instr_aam() {
    encode32_helper0(Mnemonic::AAM, &vec![0xD4, 0x0A]); // AAM
    encode32_helper1(Mnemonic::AAM, Operand::Literal8(0x3), &vec![0xD4, 0x03]); // AAM 3
}

#[test]
fn instr_aas() {
   encode32_helper0(Mnemonic::AAS, &vec![0x3F]); // AAS
}

#[test]
fn instr_adc() {
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x14, 0x12]); // ADC AL, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x15, 0x34, 0x12]); // ADC AX, 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x15, 0x78, 0x56, 0x34, 0x12]); // ADC EAX, 0x12345678
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x10, 0x12]); // ADC BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xD3, 0x12]); // ADC BL, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x10, 0x34, 0x12]); // ADC WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xd3, 0x34, 0x12]); // ADC BX, 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x10, 0x78, 0x56, 0x34, 0x12]); // ADC WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xd3, 0x78, 0x56, 0x34, 0x12]); // ADC EBX, 0x12345678
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x10, 0x12]); // ADC WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xd3, 0x12]); // ADC BX, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x10, 0x18]); // ADC BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x10, 0xCB]); // ADC BL, CL
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x11, 0x18]); // ADC WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x11, 0xcb]); // ADC BX, CX
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x11, 0x18]); // ADC DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x11, 0xcb]); // ADC EBX, ECX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x12, 0x18]); // ADC BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x13, 0x18]); // ADC BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x13, 0x18]); // ADC EBX, DWORD PTR [EAX]
}

#[test]
fn instr_adx() {
    encode32_helper2(Mnemonic::ADCX, Operand::Direct(Reg::EBX), Operand::Direct(Reg::EAX), &vec![0x66, 0x0F, 0x38, 0xF6, 0xD8]); // ADCX EBX, EAX
    encode32_helper2(Mnemonic::ADCX, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x0F, 0x38, 0xF6, 0x18]); // ADCX EBX, DWORD PTR [EAX]
}

#[test]
fn instr_add() {
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x04, 0x12]); // ADD AL, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x05, 0x34, 0x12]); // ADD AX, 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x05, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xC3, 0x12]); // ADD BL, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x00, 0x12]); // ADD BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xC3, 0x34, 0x12]); // ADD BX, 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x00, 0x34, 0x12]); // ADD WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xC3, 0x78, 0x56, 0x34, 0x12]); // ADD EBX, 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x00, 0x78, 0x56, 0x34, 0x12]); // ADD DWORD PTR [EAX], 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xC3, 0x12]); // ADD BX, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x00, 0x12]); // ADD WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Literal8(0x12), &vec![0x83, 0xC3, 0x12]); // ADD EBX, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x83, 0x00, 0x12]); // ADD DWORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x00, 0xCB]); // ADD BL, CL
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x00, 0x18]); // ADD BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x01, 0xCB]); // ADD BX, CX
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x01, 0x18]); // ADD WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x01, 0xCB]); // ADD EBX, ECX
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x01, 0x18]); // ADD DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x02, 0x18]); // ADD BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x03, 0x18]); // ADD BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x03, 0x18]); // ADD EBX, DWORD PTR [EAX]
}


#[test]
fn instr_addpd() {
    encode32_helper2(Mnemonic::ADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x58, 0xCA]); // ADDPD XMM1, XMM2
    encode32_helper2(Mnemonic::ADDPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x58, 0x08]); // ADDPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x58, 0xCB]); // ADDPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x58, 0x08]); // ADDPD XMM1, XMM2, XMMWORD [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x58, 0xCB]); // ADDPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x58, 0x08]); // VADDPD YMM1, YMM2, YMMWORD [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0x48, 0x58, 0xCB]); // ADDPD ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x48, 0x58, 0x08]); // VADDPD YMM1, YMM2, YMMWORD [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0x4B, 0x58, 0xCB]); // VADDPD ZMM1 {K3}, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VADDPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0xC8, 0x58, 0xCb]); // VADDPD ZMM1 {Z}, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x58, 0x58, 0x08]); // VADDPD ZMM1, ZMM2, QWORD PTR [EAX] {1to8}
    encode32_helper3_flags(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Zero),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xED, 0x78, 0x58, 0xCB]); // VADDPD ZMM1 {K3}, ZMM2, ZMM3

}

#[test]
fn instr_addps() {
    encode32_helper2(Mnemonic::ADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x58, 0xCA]); // ADDPS XMM1, XMM2
    encode32_helper2(Mnemonic::ADDPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x58, 0x08]); // ADDPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x58, 0x08]); // VADDPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x58, 0x08]); // VADDPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x18, 0x58, 0x08]); // VADDPS XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VADDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x38, 0x58, 0x08]); // VADDPS YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VADDPS, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x58, 0x58, 0x08]); // VADDPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_addsd() {
    encode32_helper2(Mnemonic::ADDSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x58, 0xCA]); // ADDSD XMM1, XMM2
    encode32_helper2(Mnemonic::ADDSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x58, 0x08]); // ADDSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x58, 0xCB]); // VADDSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x58, 0x08]); // VADDSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xEF, 0x0A, 0x58, 0xCB]); // VADDSD XMM1 {K2}, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xEF, 0x88, 0x58, 0x08]); // VADDSD XMM1 {K2}, XMM2, XMM3
}

#[test]
fn instr_addss() {
    encode32_helper2(Mnemonic::ADDSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x58, 0xCA]); // ADDSS XMM1, XMM2
    encode32_helper2(Mnemonic::ADDSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x58, 0x08]); // ADDSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x58, 0xCB]); // VADDSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x58, 0x08]); // VADDSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6E, 0x0A, 0x58, 0xCB]); // VADDSS XMM1 {K2}, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6E, 0x88, 0x58, 0x08]); // VADDSS XMM1 {K2}, XMM2, XMM3
}

#[test]
fn instr_addsubpd() {
    encode32_helper2(Mnemonic::ADDSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xD0, 0xCA]); // ADDSUBPD XMM1, XMM2
    encode32_helper2(Mnemonic::ADDSUBPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xD0, 0x08]); // ADDSUBPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xD0, 0xCB]); // VADDSUBPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xD0, 0x08]); // VADDSUBPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSUBPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xD0, 0xCB]); // VADDSUBPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VADDSUBPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xD0, 0x08]); // VADDSUBPD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_addsubps() {
    encode32_helper2(Mnemonic::ADDSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0xD0, 0xCA]); // ADDSUBPS XMM1, XMM2
    encode32_helper2(Mnemonic::ADDSUBPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF2, 0x0F, 0xD0, 0x08]); // ADDSUBPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0xD0, 0xCB]); // VADDSUBPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VADDSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xEB, 0xD0, 0x08]); // VADDSUBPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDSUBPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEF, 0xD0, 0xCB]); // VADDSUBPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VADDSUBPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEF, 0xD0, 0x08]); // VADDSUBPS YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_adox() {
    encode32_helper2(Mnemonic::ADOX, Operand::Direct(Reg::EBX), Operand::Direct(Reg::EAX), &vec![0xF3, 0x0F, 0x38, 0xF6, 0xD8]); // ADOX EBX, EAX
    encode32_helper2(Mnemonic::ADOX, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x38, 0xF6, 0x18]); // ADOX EBX, DWORD PTR [EAX]
}

#[test]
fn instr_aesdec() {
    encode32_helper2(Mnemonic::AESDEC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0xDE, 0xCA]); // AESDEC XMM1, XMM2
    encode32_helper2(Mnemonic::AESDEC, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0xDE, 0x08]); // AESDEC XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VAESDEC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0xDE, 0xCB]); // VAESDEC XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VAESDEC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0xDE, 0x08]); // VAESDEC XMM1, XMM2, XMMWORD PTR [EAX]
}

#[test]
fn instr_aesdeclast() {
    encode32_helper2(Mnemonic::AESDECLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0xDF, 0xCA]); // AESDECLAST XMM1, XMM2
    encode32_helper2(Mnemonic::AESDECLAST, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0xDF, 0x08]); // AESDECLAST XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VAESDECLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0xDF, 0xCB]); // VAESDECLAST XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VAESDECLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0xDF, 0x08]); // VAESDECLAST XMM1, XMM2, XMMWORD PTR [EAX]
}

#[test]
fn instr_aesenc() {
    encode32_helper2(Mnemonic::AESENC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0xDC, 0xCA]); // AESENC XMM1, XMM2
    encode32_helper2(Mnemonic::AESENC, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0xDC, 0x08]); // AESENC XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VAESENC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0xDC, 0xCB]); // VAESENC XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VAESENC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0xDC, 0x08]); // VAESENC XMM1, XMM2, XMMWORD PTR [EAX]
}

#[test]
fn instr_aesenclast() {
    encode32_helper2(Mnemonic::AESENCLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0xDD, 0xCA]); // AESENCLAST XMM1, XMM2
    encode32_helper2(Mnemonic::AESENCLAST, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0xDD, 0x08]); // AESENCLAST XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VAESENCLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0xDD, 0xCB]); // VAESENCLAST XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VAESENCLAST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0xDD, 0x08]); // VAESENCLAST XMM1, XMM2, XMMWORD PTR [EAX]
}

#[test]
fn instr_aesimc() {
    encode32_helper2(Mnemonic::AESIMC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0xDB, 0xCA]); // AESIMC XMM1, XMM2
    encode32_helper2(Mnemonic::AESIMC, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0xDB, 0x08]); // AESIMC XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VAESIMC, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC4, 0xE2, 0x79, 0xDB, 0xCA]); // VAESIMC XMM1, XMM2
    encode32_helper2(Mnemonic::VAESIMC, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x79, 0xDB, 0x08]); // VAESIMC XMM1, XMMWORD PTR [EAX]
}

#[test]
fn instr_aeskeygenassist() {
    encode32_helper3(Mnemonic::AESKEYGENASSIST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0xDF, 0xCA, 0x12]); // AESKEYGENASSIST XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::AESKEYGENASSIST, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0xDF, 0x08, 0x12]); // AESKEYGENASSIST XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::VAESKEYGENASSIST, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0xDF, 0xCA, 0x12]); // VAESKEYGENASSIST XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::VAESKEYGENASSIST, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0xDF, 0x08, 0x12]); // VAESKEYGENASSIST XMM1, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_and() {
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x24, 0x12]); // ADD AL, 0x12
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x25, 0x34, 0x12]); // ADD AX, 0x1234
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x25, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, 0x12345678
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x20, 0x12]); // ADD BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x20, 0x34, 0x12]); // ADD WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x20, 0x78, 0x56, 0x34, 0x12]); // ADD DWORD PTR [EAX], 0x12345678
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x20, 0x12]); // ADD WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x83, 0x20, 0x12]); // ADD DWORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x20, 0x18]); // ADD BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x21, 0x18]); // ADD WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x21, 0x18]); // ADD DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x20, 0xCB]); // ADD BL, CL
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x21, 0xCB]); // ADD BX, CX
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x21, 0xCB]); // ADD EBX, ECX
}

#[test]
fn instr_andn() {
    encode32_helper3(Mnemonic::ANDN, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0xC4, 0xE2, 0x60, 0xF2, 0xC1]); // ANDN EAX, EBX, ECX
    encode32_helper3(Mnemonic::ANDN, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Indirect(Reg::ECX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x60, 0xF2, 0x01]); // ANDN EAX, EBX, DWORD PTR [ECX]
}

#[test]
fn instr_andpd() {
    encode32_helper2(Mnemonic::ANDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x54, 0xCA]); // ANDPD XMM1, XMM2
    encode32_helper2(Mnemonic::ANDPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x54, 0x08]); // ANDPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x54, 0xCB]); // VANDPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x54, 0x08]); // VANDPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x54, 0xCB]); // VANDPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x54, 0x08]); // VANDPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xED, 0x88, 0x54, 0xCB]); // VANDPD XMM1{Z}, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x88, 0x54, 0x08]); // VANDPD XMM1{Z}, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x98, 0x54, 0x08]); // VANDPD XMM1{Z}, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0xA8, 0x54, 0xCB]); // VANDPD YMM1{Z}, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0xA8, 0x54, 0x08]); // VANDPD YMM1{Z}, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0xB8, 0x54, 0x08]); // VANDPD YMM1{Z}, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0xC8, 0x54, 0xCB]); // VANDPD ZMM1{Z}, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0xC8, 0x54, 0x08]); // VANDPD ZMM1{Z}, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0xD8, 0x54, 0x08]); // VANDPD ZMM1{Z}, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_andps() {
    encode32_helper2(Mnemonic::ANDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x54, 0xCA]); // ANDPS XMM1, XMM2
    encode32_helper2(Mnemonic::ANDPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x54, 0x08]); // ANDPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x54, 0xCB]); // VANDPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x54, 0x08]); // VANDPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x54, 0xCB]); // VANDPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x54, 0x08]); // VANDPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6C, 0x88, 0x54, 0xCB]); // VANDPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6C, 0x88, 0x54, 0x08]); // VANDPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x98, 0x54, 0x08]); // VANDPS XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6C, 0xA8, 0x54, 0xCB]); // VANDPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6C, 0xA8, 0x54, 0x08]); // VANDPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0xB8, 0x54, 0x08]); // VANDPS YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6C, 0xC8, 0x54, 0xCB]); // VANDPS ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6C, 0xC8, 0x54, 0x08]); // VANDPS ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0xD8, 0x54, 0x08]); // VANDPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_andnpd() {
    encode32_helper2(Mnemonic::ANDNPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x55, 0xCA]); // ANDNPD XMM1, XMM2
    encode32_helper2(Mnemonic::ANDNPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x55, 0x08]); // ANDNPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x55, 0xCB]); // VANDNPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDNPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x55, 0x08]); // VANDNPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x55, 0xCB]); // VANDNPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDNPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x55, 0x08]); // VANDNPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xED, 0x88, 0x55, 0xCB]); // VANDNPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x88, 0x55, 0x08]); // VANDNPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x98, 0x55, 0x08]); // VANDNPD XMM1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0xA8, 0x55, 0xCB]); // VANDNPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0xA8, 0x55, 0x08]); // VANDNPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0xB8, 0x55, 0x08]); // VANDNPD YMM1, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0xC8, 0x55, 0xCB]); // VANDNPD ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0xC8, 0x55, 0x08]); // VANDNPD ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPD, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0xD8, 0x55, 0x08]); // VANDNPD ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_andnps() {
    encode32_helper2(Mnemonic::ANDNPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x55, 0xCA]); // ANDNPS XMM1, XMM2
    encode32_helper2(Mnemonic::ANDNPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x55, 0x08]); // ANDNPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x55, 0xCB]); // VANDNPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDNPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x55, 0x08]); // VANDNPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x55, 0xCB]); // VANDNPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDNPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x55, 0x08]); // VANDNPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6C, 0x88, 0x55, 0xCB]); // VANDNPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6C, 0x88, 0x55, 0x08]); // VANDNPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x98, 0x55, 0x08]); // VANDNPS XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6C, 0xA8, 0x55, 0xCB]); // VANDNPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6C, 0xA8, 0x55, 0x08]); // VANDNPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0xB8, 0x55, 0x08]); // VANDNPS YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6C, 0xC8, 0x55, 0xCB]); // VANDNPS ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6C, 0xC8, 0x55, 0x08]); // VANDNPS ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VANDNPS, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0xD8, 0x55, 0x08]); // VANDNPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_arpl() {
    encode32_helper2(Mnemonic::ARPL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x63, 0x18]); // ARPL WORD PTR [EAX], BX
}

#[test]
fn instr_blendpd() {
    encode32_helper3(Mnemonic::BLENDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0D, 0xCA, 0x12]); // BLENDPD XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::BLENDPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0D, 0x08, 0x12]); // BLENDPD XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VBLENDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0D, 0xCB, 0x12]); // VBLENDPD XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VBLENDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0D, 0x08, 0x12]); // VBLENDPD XMM1, XMM2, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VBLENDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0D, 0xCB, 0x12]); // VBLENDPD YMM1, YMM2, YMM3, 0x12
    encode32_helper4(Mnemonic::VBLENDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0D, 0x08, 0x12]); // VBLENDPD YMM1, YMM2, YMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_bextr() {
    encode32_helper3(Mnemonic::BEXTR, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0xC4, 0xE2, 0x70, 0xF7, 0xC3]); // BEXTR EAX, EBX, ECX
    encode32_helper3(Mnemonic::BEXTR, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), Operand::Direct(Reg::ECX), &vec![0xC4, 0xE2, 0x70, 0xF7, 0x03]); // BEXTR EAX, DWORD PTR [EBX], ECX
}

fn instr_plendps() {
    encode32_helper3(Mnemonic::BLENDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0C, 0xCA, 0x12]); // BLENDPS XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::BLENDPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0C, 0x08, 0x12]); // BLENDPS XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VBLENDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0C, 0xCB, 0x12]); // VBLENDPS XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VBLENDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0C, 0x08, 0x12]); // VBLENDPS XMM1, XMM2, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VBLENDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0C, 0xCB, 0x12]); // VBLENDPS YMM1, YMM2, YMM3, 0x12
    encode32_helper4(Mnemonic::VBLENDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0C, 0x08, 0x12]); // VBLENDPS YMM1, YMM2, YMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_blendvpd() {
    // TODO Implicit XMM0 operand 3?
    encode32_helper2(Mnemonic::BLENDVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x15, 0xCA]); // BLENDVPD XMM1, XMM20
    encode32_helper2(Mnemonic::BLENDVPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x15, 0x08]); // BLENDVPD XMM1, XMMWORD PTR [EAX]
    encode32_helper4(Mnemonic::VBLENDVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4B, 0xCB, 0x40]); // VBLENDVPD XMM1, XMM2, XMM3, XMM4
    encode32_helper4(Mnemonic::VBLENDVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4B, 0x08, 0x40]); // VBLENDVPD XMM1, XMM2, XMMWORD PTR [EAX], XMM4
    encode32_helper4(Mnemonic::VBLENDVPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4B, 0xCB, 0x40]); // VBLENDVPD YMM1, YMM2, YMM3, YMM4
    encode32_helper4(Mnemonic::VBLENDVPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4B, 0x08, 0x40]); // VBLENDVPD YMM1, YMM2, YMMWORD PTR [EAX], YMM4
}

#[test]
fn instr_blendvps() {
    // TODO Implicit XMM0 operand 3?
    encode32_helper2(Mnemonic::BLENDVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x14, 0xCA]); // BLENDVPS XMM1, XMM2
    encode32_helper2(Mnemonic::BLENDVPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x14, 0x08]); // BLENDVPS XMM1, XMMWORD PTR [EAX]
    encode32_helper4(Mnemonic::VBLENDVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4A, 0xCB, 0x40]); // VBLENDVPS XMM1, XMM2, XMM3, XMM4
    encode32_helper4(Mnemonic::VBLENDVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4A, 0x08, 0x40]); // VBLENDVPS XMM1, XMM2, XMMWORD PTR [EAX], XMM4
    encode32_helper4(Mnemonic::VBLENDVPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4A, 0xCB, 0x40]); // VBLENDVPS YMM1, YMM2, YMM3, YMM4
    encode32_helper4(Mnemonic::VBLENDVPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4A, 0x08, 0x40]); // VBLENDVPS YMM1, YMM2, YMMWORD PTR [EAX], YMM4
}

#[test]
fn instr_blsi() {
    encode32_helper2(Mnemonic::BLSI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0xC4, 0xE2, 0x78, 0xF3, 0xDB]); // BLSI EAX, EBX
    encode32_helper2(Mnemonic::BLSI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x78, 0xF3, 0x1B]); // BLSI EAX, DWORD PTR [EBX]
}

#[test]
fn instr_blsmsk() {
    encode32_helper2(Mnemonic::BLSMSK, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0xC4, 0xE2, 0x78, 0xF3, 0xD3]); // BLSMSK EAX, EBX
    encode32_helper2(Mnemonic::BLSMSK, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x78, 0xF3, 0x13]); // BLSMSK EAX, DWORD PTR [EBX]
}

#[test]
fn instr_blsr() {
    encode32_helper2(Mnemonic::BLSR, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0xC4, 0xE2, 0x78, 0xF3, 0xCB]); // BLSR EAX, EBX
    encode32_helper2(Mnemonic::BLSR, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x78, 0xF3, 0x0B]); // BLSR EAX, DWORD PTR [EBX]
}

#[test]
fn instr_bndcl() {
    encode32_helper2(Mnemonic::BNDCL, Operand::Direct(Reg::BND0), Operand::Direct(Reg::EAX), &vec![0xF3, 0x0F, 0x1A, 0xC0]); // BNDCL BND0, EAX
    encode32_helper2(Mnemonic::BNDCL, Operand::Direct(Reg::BND0), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x1A, 0x00]); // BNDCL BND0, EAX
}

#[test]
fn instr_bndcu() {
    encode32_helper2(Mnemonic::BNDCU, Operand::Direct(Reg::BND0), Operand::Direct(Reg::EAX), &vec![0xF2, 0x0F, 0x1A, 0xC0]); // BNDCU BND0, EAX
    encode32_helper2(Mnemonic::BNDCU, Operand::Direct(Reg::BND0), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF2, 0x0F, 0x1A, 0x00]); // BNDCU BND0, EAX
}

#[test]
fn instr_bndcn() {
    encode32_helper2(Mnemonic::BNDCN, Operand::Direct(Reg::BND0), Operand::Direct(Reg::EAX), &vec![0xF2, 0x0F, 0x1B, 0xC0]); // BNDCN BND0, EAX
    encode32_helper2(Mnemonic::BNDCN, Operand::Direct(Reg::BND0), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF2, 0x0F, 0x1B, 0x00]); // BNDCN BND0, EAX
}

#[test]
fn instr_bndldx() {
    encode32_helper2(Mnemonic::BNDLDX, Operand::Direct(Reg::BND0), Operand::Indirect(Reg::EAX, None, None), &vec![0x0F, 0x1A, 0x00]); // BNDLDX BND0, [EAX]
    encode32_helper2(Mnemonic::BNDLDX, Operand::Direct(Reg::BND0), Operand::IndirectScaledIndexed(Reg::EAX, Reg::EBX, RegScale::One, None, None), &vec![0x0F, 0x1A, 0x04, 0x18]); // BNDLDX BND0, [EAX+EBX*1]
}

#[test]
fn instr_bndmov() {
    encode32_helper2(Mnemonic::BNDMOV, Operand::Direct(Reg::BND1), Operand::Direct(Reg::BND2), &vec![0x66, 0x0F, 0x1A, 0xCA]); // BNDMOV BND1, BND2
    encode32_helper2(Mnemonic::BNDMOV, Operand::Direct(Reg::BND1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x1A, 0x08]); // BNDMOV BND1, QWORD PTR [EAX]
}

#[test]
fn instr_bndstx() {
    encode32_helper2(Mnemonic::BNDSTX, Operand::IndirectScaledIndexed(Reg::EAX, Reg::EBX, RegScale::One, None, None), Operand::Direct(Reg::BND3), &vec![0x0F, 0x1B, 0x1C, 0x18]); // BNDSTX [EAX+EBX*1], BND3
}

#[test]
fn instr_bound() {
    encode32_helper2(Mnemonic::BOUND, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x62, 0x18]); // BOUND BX, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::BOUND, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0x18]); // BOUND EBX, QWORD PTR [EAX]
}

#[test]
fn instr_bsf() {
    encode32_helper2(Mnemonic::BSF, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0f, 0xbc, 0x18]); // BSF BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::BSF, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0f, 0xbc, 0x18]); // BSF EBX, DWORD PTR [EAX]
}

#[test]
fn instr_bsr() {
    encode32_helper2(Mnemonic::BSR, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0f, 0xbd, 0x18]); // BSF BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::BSR, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0f, 0xbd, 0x18]); // BSF EBX, DWORD PTR [EAX]
}

#[test]
fn instr_bswap() {
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EAX), &vec![0x0f, 0xc8]); // BSWAP EAX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EBX), &vec![0x0f, 0xcb]); // BSWAP EBX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ECX), &vec![0x0f, 0xc9]); // BSWAP ECX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EDX), &vec![0x0f, 0xca]); // BSWAP EDX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EDI), &vec![0x0f, 0xcf]); // BSWAP EDI
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ESI), &vec![0x0f, 0xce]); // BSWAP ESI
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ESP), &vec![0x0f, 0xcc]); // BSWAP ESP
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EBP), &vec![0x0f, 0xcd]); // BSWAP EBP
}

#[test]
fn instr_bt() {
    encode32_helper2(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xA3, 0x18]); // BT WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xA3, 0x18]); // BT DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x20, 0x12]); // BT WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x20, 0x12]); // BT DWORD PTR [EAX], 0x12
}

#[test]
fn instr_btc() {
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xBB, 0x18]); // BTC WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xBB, 0x18]); // BTC DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x38, 0x12]); // BTC WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x38, 0x12]); // BTC DWORD PTR [EAX], 0x12
}

#[test]
fn instr_btr() {
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xB3, 0x18]); // BTR WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xB3, 0x18]); // BTR DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x30, 0x12]); // BTR WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x30, 0x12]); // BTR DWORD PTR [EAX], 0x12
}

#[test]
fn instr_bts() {
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xAB, 0x18]); // BTS WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xAB, 0x18]); // BTS DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x28, 0x12]); // BTS WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x28, 0x12]); // BTS DWORD PTR [EAX], 0x12
}

#[test]
fn instr_bzhi() {
    encode32_helper3(Mnemonic::BZHI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0xC4, 0xE2, 0x70, 0xF5, 0xC3]); // BZHI EAX, EBX, ECX
    encode32_helper3(Mnemonic::BZHI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), Operand::Direct(Reg::ECX), &vec![0xC4, 0xE2, 0x70, 0xF5, 0x03]); // BZHI EAX, DWORD PTR [EBX], ECX
}

#[test]
fn instr_call() {
    // TODO It seems many assemblers factor in instruction size on relative call instructions.
    // Maybe we should do this?
    encode32_helper1(Mnemonic::CALL, Operand::Literal16(0x1234), &vec![0x66, 0xe8, 0x34, 0x12]); // CALL 0x1234
    encode32_helper1(Mnemonic::CALL, Operand::Literal32(0x12345678), &vec![0xe8, 0x78, 0x56, 0x34, 0x12]); // CALL 0x12345678
    encode32_helper1(Mnemonic::CALL, Operand::Direct(Reg::AX), &vec![0x66, 0xff, 0xd0]); // CALL AX
    encode32_helper1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xff, 0x10]); // CALL WORD PTR [EAX]
    encode32_helper1(Mnemonic::CALL, Operand::Direct(Reg::EAX), &vec![0xff, 0xd0]); // CALL EAX
    encode32_helper1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xff, 0x10]); // CALL DWORD PTR [EAX]
    // TODO Assembler should auto-detect far calls (alias callf?)
    encode32_helper1(Mnemonic::CALLF, Operand::MemoryAndSegment16(0x12, 0x3456), &vec![0x66, 0x9a, 0x56, 0x34, 0x12, 0x00]); // CALL 0x12:0x3456
    encode32_helper1(Mnemonic::CALLF, Operand::MemoryAndSegment32(0x12, 0x3456789a), &vec![0x9a, 0x9a, 0x78, 0x56, 0x34, 0x12, 0x00]); // CALL 0x12:0x3456789A
}

#[test]
fn instr_cbw() {
    encode32_helper0(Mnemonic::CBW, &vec![0x66, 0x98]); // CBW
}

#[test]
fn instr_cwde() {
    encode32_helper0(Mnemonic::CWDE, &vec![0x98]); // CWDE
}

#[test]
fn instr_clac() {
    encode32_helper0(Mnemonic::CLAC,  &vec![0x0F, 0x01, 0xCA]); // CLAC 
}

#[test]
fn instr_clc() {
    encode32_helper0(Mnemonic::CLC, &vec![0xf8]); // CLC
}

#[test]
fn instr_cld() {
    encode32_helper0(Mnemonic::CLD, &vec![0xfc]); // CLD
}

#[test]
fn instr_clflush() {
    encode32_helper1(Mnemonic::CLFLUSH, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x0f, 0xae, 0x38]); // CLFLUSH BYTE PTR [EAX]
}

#[test]
fn instr_clflushopt() {
    encode32_helper1(Mnemonic::CLFLUSHOPT, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x66, 0x0F, 0xAE, 0x38]); // CLFLUSHOPT BYTE PTR [EAX]
}

#[test]
fn instr_cli() {
    encode32_helper0(Mnemonic::CLI, &vec![0xfa]); // CLI
}

#[test]
fn instr_clts() {
    encode32_helper0(Mnemonic::CLTS, &vec![0x0f, 0x06]); // CLTS
}

#[test]
fn instr_clwb() {
    encode32_helper1(Mnemonic::CLWB, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x66, 0x0F, 0xAE, 0x30]); // CLWB BYTE PTR [EAX]
}

#[test]
fn instr_cmc() {
    encode32_helper0(Mnemonic::CMC, &vec![0xf5]); // CMC
}

#[test]
fn instr_cmov() {
    fn cmov_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        encode32_helper2(mnemonic, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0x0f, primary_opcode, 0xc3]);
    }

    fn cmov_test_helper_aliased(mnemonics: &[Mnemonic], primary_opcode: u8) {
        test_aliased(mnemonics, |m| cmov_test_helper(m, primary_opcode) );
    }

    // OF
    cmov_test_helper(Mnemonic::CMOVO, 0x40);

    // !OF
    cmov_test_helper(Mnemonic::CMOVNO, 0x41);

    // CF
    cmov_test_helper_aliased(&[Mnemonic::CMOVB, Mnemonic::CMOVNAE, Mnemonic::CMOVC], 0x42);

    // !CF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNB, Mnemonic::CMOVAE, Mnemonic::CMOVNC], 0x43);
    
    // ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVZ, Mnemonic::CMOVE], 0x44);

    // !ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNZ, Mnemonic::CMOVNE], 0x45);
    
    // CF+ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVBE, Mnemonic::CMOVNA], 0x46);

    // !CF*!ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNBE, Mnemonic::CMOVA], 0x47);

    // SF
    cmov_test_helper(Mnemonic::CMOVS, 0x48);
    
    // !SF
    cmov_test_helper(Mnemonic::CMOVNS, 0x49);

    // PF
    cmov_test_helper_aliased(&[Mnemonic::CMOVP, Mnemonic::CMOVPE], 0x4A);

    // !PF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNP, Mnemonic::CMOVPO], 0x4B);

    // SF!=OF
    cmov_test_helper_aliased(&[Mnemonic::CMOVL, Mnemonic::CMOVNGE], 0x4C);

    // SF=OF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNL, Mnemonic::CMOVGE], 0x4D);

    // ZF+(SF!=OF)
    cmov_test_helper_aliased(&[Mnemonic::CMOVLE, Mnemonic::CMOVNG], 0x4E);

    // !ZF*(SF=OF)
    cmov_test_helper_aliased(&[Mnemonic::CMOVNLE, Mnemonic::CMOVG], 0x4F);
}

#[test]
fn instr_cmp() {
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x3C, 0x12]); // CMP AL, 0x12
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x3D, 0x34, 0x12]); // CMP AX, 0x1234
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x3D, 0x78, 0x56, 0x34, 0x12]); // CMP EAX, 0x12345678
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xFB, 0x12]); // CMP BL, 0x12
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xFB, 0x34, 0x12]); // CMP BX, 0x1234
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xFB, 0x78, 0x56, 0x34, 0x12]); // CMP EAX, 0x12345678
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xFB, 0x12]); // CMP BX, 0x12
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Literal8(0x12), &vec![0x83, 0xFB, 0x12]); // CMP EBX, 0x12
    encode32_helper2(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x38, 0x18]); // CMP BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x39, 0x18]); // CMP WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x39, 0x18]); // CMP DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x3A, 0x18]); // CMP BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x3B, 0x18]); // CMP BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x3B, 0x18]); // CMP EBX, DWORD PTR [EAX]
}

#[test]
fn instr_cmppd() {
    encode32_helper3(Mnemonic::CMPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0xFF), &vec![0x66, 0x0F, 0xC2, 0xCA, 0xFF]); // CMPPD XMM1, XMM2, 0xFF
    encode32_helper3(Mnemonic::CMPPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0xFF), &vec![0x66, 0x0F, 0xC2, 0x08, 0xFF]); // CMPPD XMM1, XMMWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xE9, 0xC2, 0xCB, 0xFF]); // VCMPPD XMM1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0xFF), &vec![0xC5, 0xE9, 0xC2, 0x08, 0xFF]); // VCMPPD XMM1, XMM2, XMMWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xED, 0xC2, 0xCB, 0xFF]); // VCMPPD YMM1, YMM2, YMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0xFF), &vec![0xC5, 0xED, 0xC2, 0x08, 0xFF]); // VCMPPD YMM1, YMM2, YMMWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xED, 0x08, 0xC2, 0xCB, 0xFF]); // VCMPPD K1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xED, 0x0A, 0xC2, 0x08, 0xFF]); // VCMPPD K1, XMM2, XMMWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xED, 0x18, 0xC2, 0x08, 0xFF]); // VCMPPD K1, XMM2, QWORD PTR [EAX]{1to2}, 0xFF
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::K1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xED, 0x28, 0xC2, 0xCB, 0xFF]); // VCMPPD K1, YMM2, YMM3, 0xFF
    // TODO {sae}?
    encode32_helper4(Mnemonic::VCMPPD, Operand::Direct(Reg::K1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xED, 0x48, 0xC2, 0xCB, 0xFF]); // VCMPPD K1, ZMM2, ZMM3, 0xFF
}

#[test]
fn instr_cmpps() {
    encode32_helper3(Mnemonic::CMPPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0xFF), &vec![0x0F, 0xC2, 0xCA, 0xFF]); // CMPPS XMM1, XMM2, 0xFF
    encode32_helper4(Mnemonic::VCMPPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xE8, 0xC2, 0xCB, 0xFF]); // VCMPPS XMM1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xEC, 0xC2, 0xCB, 0xFF]); // VCMPPS YMM1, YMM2, YMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPS, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0x6C, 0x08, 0xC2, 0xCB, 0xFF]); // VCMPPS K1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPPS, Operand::Direct(Reg::K1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0x6C, 0x28, 0xC2, 0x08, 0xFF]); // VCMPPS K1, YMM2, YMMWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPPS, Operand::Direct(Reg::K1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0x6C, 0x58, 0xC2, 0x08, 0xFF]); // VCMPPS K1, ZMM2, DWORD PTR [EAX]{1to16}, 0xFF
}

#[test]
fn instr_cmps() {
    encode32_helper2(Mnemonic::CMPS, Operand::Indirect(Reg::EDI, Some(OperandSize::Byte), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Byte), Some(SegmentReg::DS)), &vec![0xA6]); // CMPS BYTE PTR DS:[EDI], BYTE PTR ES:[ESI]
    encode32_helper2(Mnemonic::CMPS, Operand::Indirect(Reg::EDI, Some(OperandSize::Word), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Word), Some(SegmentReg::DS)), &vec![0x66, 0xA7]); // CMPS WORD PTR DS:[EDI], WORD PTR ES:[ESI]
    encode32_helper2(Mnemonic::CMPS, Operand::Indirect(Reg::EDI, Some(OperandSize::Dword), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Dword), Some(SegmentReg::DS)), &vec![0xA7]); // CMPS DWORD PTR DS:[EDI], DWORD PTR ES:[ESI]
}

#[test]
fn instr_cmpsb() {
    encode32_helper0(Mnemonic::CMPSB, &vec![0xA6]); // CMPSB
}

#[test]
fn instr_cmpsw() {
    encode32_helper0(Mnemonic::CMPSW, &vec![0x66, 0xA7]); // CMPSW
}

#[test]
fn instr_cmpsd_string() {
    encode32_helper0(Mnemonic::CMPSD, &vec![0xA7]); // CMPSD
}

#[test]
fn instr_cmpsd_sse() {
   encode32_helper3(Mnemonic::CMPSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0xFF), &vec![0xF2, 0x0F, 0xC2, 0xCA, 0xFF]); // CMPSD XMM1, XMM2, 0xFF
   encode32_helper3(Mnemonic::CMPSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Literal8(0xFF), &vec![0xF2, 0x0F, 0xC2, 0x08, 0xFF]); // CMPSD XMM1, QWORD PTR [EAX], 0xFF
   encode32_helper4(Mnemonic::VCMPSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xEB, 0xC2, 0xCB, 0xFF]); // VCMPSD XMM1, XMM2, XMM3, 0xFF
   encode32_helper4(Mnemonic::VCMPSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Literal8(0xFF), &vec![0xC5, 0xEB, 0xC2, 0x08, 0xFF]); // VCMPSD XMM1, XMM2, QWORD PTR [EAX], 0xFF
   encode32_helper4(Mnemonic::VCMPSD, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xEF, 0x08, 0xC2, 0xCB, 0xFF]); // VCMPSD K1, XMM2, XMM3, 0xFF
   encode32_helper4(Mnemonic::VCMPSD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0xEF, 0x0A, 0xC2, 0x08, 0xFF]); // VCMPSD K1, XMM2, QWORD PTR [EAX], 0xFF
}

#[test]
fn instr_cmpss() {
    encode32_helper3(Mnemonic::CMPSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0xFF), &vec![0xF3, 0x0F, 0xC2, 0xCA, 0xFF]); // CMPSS XMM1, XMM2, 0xFF
    encode32_helper3(Mnemonic::CMPSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0xFF), &vec![0xF3, 0x0F, 0xC2, 0x08, 0xFF]); // CMPSS XMM1, DWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0xC5, 0xEA, 0xC2, 0xCB, 0xFF]); // VCMPSS XMM1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0xFF), &vec![0xC5, 0xEA, 0xC2, 0x08, 0xFF]); // VCMPSS XMM1, XMM2, DWORD PTR [EAX], 0xFF
    encode32_helper4(Mnemonic::VCMPSS, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0x6E, 0x08, 0xC2, 0xCB, 0xFF]); // VCMPSS K1, XMM2, XMM3, 0xFF
    encode32_helper4(Mnemonic::VCMPSS, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0xFF), &vec![0x62, 0xF1, 0x6E, 0x0A, 0xC2, 0x08, 0xFF]); // VCMPSS K1, XMM2, DWORD PTR [EAX], 0xFF
}

#[test]
fn instr_cmpxchg() {
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::AL), Operand::Direct(Reg::BL), &vec![0x0F, 0xB0, 0xD8]); // CMPXCHG AL, BL
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::AX), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xB1, 0xD8]); // CMPXCHG AX, BX
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0x0F, 0xB1, 0xD8]); // CMPXCHG EAX, EBX
}

#[test]
fn instr_comisd() {
    encode32_helper2(Mnemonic::COMISD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x2F, 0xCA]); // COMISD XMM1, XMM2
    encode32_helper2(Mnemonic::COMISD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x2F, 0x08]); // COMISD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCOMISD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0x2F, 0xCA]); // VCOMISD XMM1, XMM2
    encode32_helper2(Mnemonic::VCOMISD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xF9, 0x2F, 0x08]); // VCOMISD XMM1, QWORD PTR [EAX]
    encode32_helper2_flags(Mnemonic::VCOMISD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xFD, 0x18, 0x2F, 0xCA]); // VCOMISD XMM1, QWORD PTR [EAX]
}

#[test]
fn instr_comiss() {
    encode32_helper2(Mnemonic::COMISS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x2F, 0xCA]); // COMISS XMM1, XMM2
    encode32_helper2(Mnemonic::COMISS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0F, 0x2F, 0x08]); // COMISS XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCOMISS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF8, 0x2F, 0xCA]); // VCOMISS XMM1, XMM2
    encode32_helper2(Mnemonic::VCOMISS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xF8, 0x2F, 0x08]); // VCOMISS XMM1, DWORD PTR [EAX]
    encode32_helper2_flags(Mnemonic::VCOMISS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7C, 0x18, 0x2F, 0xCA]); // VCOMISS XMM1, QWORD PTR [EAX]

}

#[test]
fn instr_cvtdq2pd() {
    encode32_helper2(Mnemonic::CVTDQ2PD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0xE6, 0xCA]); // CVTDQ2PD XMM1, XMM2
    encode32_helper2(Mnemonic::CVTDQ2PD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF3, 0x0F, 0xE6, 0x08]); // CVTDQ2PD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0xE6, 0xCA]); // VCVTDQ2PD XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xFA, 0xE6, 0x08]); // VCVTDQ2PD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFE, 0xE6, 0xCA]); // VCVTDQ2PD YMM1, XMM2
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFE, 0xE6, 0x08]); // VCVTDQ2PD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::XMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0x18, 0xE6, 0x08]); // VCVTDQ2PD XMM1, DWORD PTR [EAX]{1to2}
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::YMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0x38, 0xE6, 0x08]); // VCVTDQ2PD YMM1, DWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTDQ2PD, Operand::Direct(Reg::ZMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0x58, 0xE6, 0x08]); // VCVTDQ2PD ZMM1, DWORD PTR [EAX]{1to8}
}

#[test]
fn instr_cvtdq2ps() {
    encode32_helper2(Mnemonic::CVTDQ2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x5B, 0xCA]); // CVTDQ2PS XMM1, XMM2
    encode32_helper2(Mnemonic::CVTDQ2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x5B, 0x08]); // CVTDQ2PS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF8, 0x5B, 0xCA]); // VCVTDQ2PS XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF8, 0x5B, 0x08]); // VCVTDQ2PS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFC, 0x5B, 0x08]); // VCVTDQ2PS YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::XMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x18, 0x5B, 0x08]); // VCVTDQ2PS XMM1, DWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::YMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x38, 0x5B, 0x08]); // VCVTDQ2PS YMM1, DWORD PTR [EAX]{1to8}
    encode32_helper2(Mnemonic::VCVTDQ2PS, Operand::Direct(Reg::ZMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x58, 0x5B, 0x08]); // VCVTDQ2PS ZMM1, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_cvtpd2dq() {
    encode32_helper2(Mnemonic::CVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0xE6, 0xCA]); // CVTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::CVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF2, 0x0F, 0xE6, 0x08]); // CVTPD2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFB, 0xE6, 0xCA]); // VCVTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFB, 0xE6, 0x08]); // VCVTPD2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFF, 0xE6, 0xCA]); // VCVTPD2DQ XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFF, 0xE6, 0x08]); // VCVTPD2DQ XMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFF, 0x88, 0xE6, 0xCA]); // VCVTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFF, 0x88, 0xE6, 0xCA]); // VCVTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFF, 0x98, 0xE6, 0x08]); // VCVTPD2DQ XMM1, QWORD PTR [EAX]{1to2}
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFF, 0xE6, 0xCA]); // VCVTPD2DQ XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::XMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFF, 0x38, 0xE6, 0x08]); // VCVTPD2DQ XMM1, QWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0xFF, 0x48, 0xE6, 0xCA]); // VCVTPD2DQ YMM1, ZMM2
    encode32_helper2(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::YMM1), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFF, 0x58, 0xE6, 0x08]); // VCVTPD2DQ YMM1, QWORD PTR [EAX]{1to8}
    encode32_helper2_flags(Mnemonic::VCVTPD2DQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::ZMM2), InstructionFlags {
        rounding_mode: Some(RoundingMode::Zero),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xFF, 0x78, 0xE6, 0xCA]); // VCVTPD2DQ YMM1, ZMM2, {rz-sae}
}

#[test]
fn instr_cvtpd2pi() {
    encode32_helper2(Mnemonic::CVTPD2PI, Operand::Direct(Reg::MM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x2D, 0xCA]); // CVTPD2PI MM1, XMM2
    encode32_helper2(Mnemonic::CVTPD2PI, Operand::Direct(Reg::MM4), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x2D, 0x20]); // CVTPD2PI MM4, XMMWORD PTR [EAX]
}

#[test]
fn instr_cvtpd2ps() {
    encode32_helper2(Mnemonic::CVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x5A, 0xCA]); // CVTPD2PS XMM1, XMM2
    encode32_helper2(Mnemonic::CVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x5A, 0x08]); // CVTPD2PS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0x5A, 0xCA]); // VCVTPD2PS XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF9, 0x5A, 0x08]); // VCVTPD2PS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFD, 0x5A, 0xCA]); // VCVTPD2PS XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFD, 0x5A, 0x08]); // VCVTPD2PS XMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x5A, 0xCA]); // VCVTPD2PS XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x5A, 0x08]); // VCVTPD2PS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x1A, 0x5A, 0x08]); // VCVTPD2PS XMM1, QWORD PTR [EAX]{1to2}
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x5A, 0xCA]); // VCVTPD2PS XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x5A, 0x08]); // VCVTPD2PS XMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x3A, 0x5A, 0x08]); // VCVTPD2PS XMM1, QWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x5A, 0xCA]); // VCVTPD2PS YMM1, ZMM2
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x5A, 0x08]); // VCVTPD2PS YMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPD2PS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x5A, 0x5A, 0x08]); // VCVTPD2PS YMM1, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_cvtpi2pd() {
    encode32_helper2(Mnemonic::CVTPI2PD, Operand::Direct(Reg::XMM2), Operand::Direct(Reg::MM5), &vec![0x66, 0x0F, 0x2A, 0xD5]); // CVTPI2PD XMM2, MM5
    encode32_helper2(Mnemonic::CVTPI2PD, Operand::Direct(Reg::XMM4), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x2A, 0x20]); // CVTPI2PD XMM4, QWORD PTR [EAX]
}

#[test]
fn instr_cvtpi2ps() {
    encode32_helper2(Mnemonic::CVTPI2PS, Operand::Direct(Reg::XMM2), Operand::Direct(Reg::MM5), &vec![0x0F, 0x2A, 0xD5]); // CVTPI2PS XMM2, MM5
    encode32_helper2(Mnemonic::CVTPI2PS, Operand::Direct(Reg::XMM3), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x2A, 0x18]); // CVTPI2PS XMM3, QWORD PTR [EAX]
}

#[test]
fn instr_cvtps2dq() {
    encode32_helper2(Mnemonic::CVTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x5B, 0xCA]); // CVTPS2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::CVTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x5B, 0x08]); // CVTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0x5B, 0xCA]); // VCVTPS2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF9, 0x5B, 0x08]); // VCVTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFD, 0x5B, 0xCA]); // VCVTPS2DQ YMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFD, 0x5B, 0x08]); // VCVTPS2DQ YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7D, 0x09, 0x5B, 0xCA]); // VCVTPS2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7D, 0x09, 0x5B, 0x08]); // VCVTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7D, 0x19, 0x5B, 0x08]); // VCVTPS2DQ XMM1, DWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7D, 0x29, 0x5B, 0xCA]); // VCVTPS2DQ YMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7D, 0x29, 0x5B, 0x08]); // VCVTPS2DQ YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7D, 0x39, 0x5B, 0x08]); // VCVTPS2DQ YMM1, DWORD PTR [EAX]{1to8}
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0x7D, 0x49, 0x5B, 0xCA]); // VCVTPS2DQ ZMM1, ZMM2
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7D, 0x49, 0x5B, 0x08]); // VCVTPS2DQ ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2DQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7D, 0x59, 0x5B, 0x08]); // VCVTPS2DQ ZMM1, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_cvtps2pd() {
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF8, 0x5A, 0xCA]); // VCVTPS2PD XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xF8, 0x5A, 0x08]); // VCVTPS2PD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFC, 0x5A, 0xCA]); // VCVTPS2PD YMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFC, 0x5A, 0x08]); // VCVTPS2PD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K7), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7C, 0x0F, 0x5A, 0xCA]); // VCVTPS2PD XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K7), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0x7C, 0x0F, 0x5A, 0x08]); // VCVTPS2PD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K7), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x1F, 0x5A, 0x08]); // VCVTPS2PD XMM1, DWORD PTR [EAX]{1to2}
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K7), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7C, 0x2F, 0x5A, 0xCA]); // VCVTPS2PD YMM1, XMM2
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K7), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7C, 0x2F, 0x5A, 0x08]); // VCVTPS2PD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K7), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x3F, 0x5A, 0x08]); // VCVTPS2PD YMM1, DWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K7), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7C, 0x4F, 0x5A, 0xCA]); // VCVTPS2PD ZMM1, YMM2
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K7), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7C, 0x4F, 0x5A, 0x08]); // VCVTPS2PD ZMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K7), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7C, 0x5F, 0x5A, 0x08]); // VCVTPS2PD ZMM1, DWORD PTR [EAX]{1to8}
    // NOTE: GAS emits [62 F1 7C 1F 5A CA] for the following test, but accepts [62 F1 7C 5F 5A CA]
    // (disassembles to the same thing). I think the 5F form makes more sense, since it indicates
    // 512-bit vector length. Apparently those bits are ignored when SAE is set?
    encode32_helper2_flags(Mnemonic::VCVTPS2PD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K7), None), Operand::Direct(Reg::YMM2), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7C, 0x5F, 0x5A, 0xCA]); // VCVTPS2PD ZMM1, YMM2
}

#[test]
fn instr_cvtps2pi() {
    encode32_helper2(Mnemonic::CVTPS2PI, Operand::Direct(Reg::MM2), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x2D, 0xD2]); // CVTPS2PI MM2, XMM2
    encode32_helper2(Mnemonic::CVTPS2PI, Operand::Direct(Reg::MM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x2D, 0x10]); // CVTPS2PI 0x2, QWORD PTR [EAX]
}

#[test]
fn instr_cvtsd2si() {
    encode32_helper2(Mnemonic::CVTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xF2, 0x0F, 0x2D, 0xC1]); // CVTSD2SI EAX, XMM1
    encode32_helper2(Mnemonic::CVTSD2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x2D, 0x03]); // CVTSD2SI EAX, QWORD PTR [EBX]
    encode32_helper2(Mnemonic::VCVTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFB, 0x2D, 0xC1]); // VCVTSD2SI EAX, XMM1
    encode32_helper2(Mnemonic::VCVTSD2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Qword), None), &vec![0xC5, 0xFB, 0x2D, 0x03]); // VCVTSD2SI EAX, QWORD PTR [EBX]
    encode32_helper2_flags(Mnemonic::VCVTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), InstructionFlags {
        rounding_mode: Some(RoundingMode::Up),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7F, 0x58, 0x2D, 0xC1]); // VCVTSD2SI EAX, XMM1, {RU-SAE}
}

#[test]
fn instr_cvtsd2ss() {
    encode32_helper2(Mnemonic::CVTSD2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x5A, 0xCA]); // CVTSD2SS XMM1, XMM2
    encode32_helper2(Mnemonic::CVTSD2SS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x5A, 0x08]); // CVTSD2SS XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSD2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x5A, 0xCB]); // VCVTSD2SS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VCVTSD2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x5A, 0x08]); // VCVTSD2SS XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSD2SS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xEF, 0x09, 0x5A, 0x08]); // VCVTSD2SS XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3_flags(Mnemonic::VCVTSD2SS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Down),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xEF, 0x39, 0x5A, 0xCB]); // VCVTSD2SS XMM1, XMM2, XMM3, {RD-SAE}
}
#[test]
fn instr_cvtsi2sd() {
    encode32_helper2(Mnemonic::CVTSI2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), &vec![0xF2, 0x0F, 0x2A, 0xC8]); // CVTSI2SD XMM1, EAX
    encode32_helper2(Mnemonic::CVTSI2SD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF2, 0x0F, 0x2A, 0x08]); // CVTSI2SD XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSI2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::EAX), &vec![0xC5, 0xEB, 0x2A, 0xC8]); // VCVTSI2SD XMM1, XMM2, EAX
    encode32_helper3(Mnemonic::VCVTSI2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEB, 0x2A, 0x08]); // VCVTSI2SD XMM1, XMM2, DWORD PTR [EAX]
}

#[test]
fn instr_cvtsi2ss() {
    encode32_helper2(Mnemonic::CVTSI2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), &vec![0xF3, 0x0F, 0x2A, 0xC8]); // CVTSI2SS XMM1, EAX
    encode32_helper2(Mnemonic::CVTSI2SS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x2A, 0x08]); // CVTSI2SS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSI2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::EAX), &vec![0xC5, 0xEA, 0x2A, 0xC8]); // VCVTSI2SS XMM1, XMM2, EAX
    encode32_helper3(Mnemonic::VCVTSI2SS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x2A, 0x08]); // VCVTSI2SS XMM1, XMM2, DWORD PTR [EAX]
    // NOTE: Intel docs say that the AVX-512 form of this instruction supports rounding modes in
    // 32-bit mode, but no assemblers seem to support this. Probably should look more into this.
}

#[test]
fn instr_cvtss2sd() {
    encode32_helper2(Mnemonic::CVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x5A, 0xCA]); // CVTSS2SD XMM1, XMM2
    encode32_helper2(Mnemonic::CVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x5A, 0x08]); // CVTSS2SD XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x5A, 0xCB]); // VCVTSS2SD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VCVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x5A, 0x08]); // VCVTSS2SD XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VCVTSS2SD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6E, 0x09, 0x5A, 0x08]); // VCVTSS2SD XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3_flags(Mnemonic::VCVTSS2SD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x6E, 0x18, 0x5A, 0xCB]); // VCVTSS2SD XMM1, XMM2, XMM3, {SAE}
}

#[test]
fn instr_cvtss2si() {
    encode32_helper2(Mnemonic::CVTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xF3, 0x0F, 0x2D, 0xC1]); // CVTSS2SI EAX, XMM1
    encode32_helper2(Mnemonic::CVTSS2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x2D, 0x03]); // CVTSS2SI EAX, DWORD PTR [EBX]
    encode32_helper2(Mnemonic::VCVTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFA, 0x2D, 0xC1]); // VCVTSS2SI EAX, XMM1
    encode32_helper2(Mnemonic::VCVTSS2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xC5, 0xFA, 0x2D, 0x03]); // VCVTSS2SI EAX, DWORD PTR [EBX]
    encode32_helper2_flags(Mnemonic::VCVTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), InstructionFlags {
        rounding_mode: Some(RoundingMode::Down),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7E, 0x38, 0x2D, 0xC1]); // VCVTSS2SI EAX, XMM1, {RD-SAE}
}

#[test]
fn instr_cvttpd2dq() {
    encode32_helper2(Mnemonic::CVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xE6, 0xCA]); // CVTTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::CVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xE6, 0x08]); // CVTTPD2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0xE6, 0xCA]); // VCVTTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF9, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFD, 0xE6, 0xCA]); // VCVTTPD2DQ XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFD, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFD, 0x09, 0xE6, 0xCA]); // VCVTTPD2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xFD, 0x09, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x19, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, QWORD PTR [EAX]{1to2}
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0xFD, 0x29, 0xE6, 0xCA]); // VCVTTPD2DQ XMM1, YMM2
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFD, 0x29, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x39, 0xE6, 0x08]); // VCVTTPD2DQ XMM1, QWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0xFD, 0x49, 0xE6, 0xCA]); // VCVTTPD2DQ YMM1, ZMM2
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xFD, 0x49, 0xE6, 0x08]); // VCVTTPD2DQ YMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFD, 0x59, 0xE6, 0x08]); // VCVTTPD2DQ YMM1, QWORD PTR [EAX]{1to8}
    // See cvttpd2pd for note on vector size bits
    encode32_helper2_flags(Mnemonic::VCVTTPD2DQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xFD, 0x59, 0xE6, 0xCA]); // VCVTTPD2DQ YMM1, ZMM2, {SAE}
}

#[test]
fn instr_cvttpd2pi() {
    encode32_helper2(Mnemonic::CVTTPD2PI, Operand::Direct(Reg::MM1), Operand::Direct(Reg::XMM0), &vec![0x66, 0x0F, 0x2C, 0xC8]); // CVTTPD2PI MM1, XMM0
    encode32_helper2(Mnemonic::CVTTPD2PI, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x2C, 0x08]); // CVTTPD2PI MM1, XMMWORD PTR [EAX]
}

#[test]
fn instr_cvttps2dq() {
    encode32_helper2(Mnemonic::CVTTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x5B, 0xCA]); // CVTTPS2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::CVTTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF3, 0x0F, 0x5B, 0x08]); // CVTTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0x5B, 0xCA]); // VCVTTPS2DQ XMM1, XMM2
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFA, 0x5B, 0x08]); // VCVTTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFE, 0x5B, 0xCA]); // VCVTTPS2DQ YMM1, YMM2
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFE, 0x5B, 0x08]); // VCVTTPS2DQ YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x7E, 0x88, 0x5B, 0xCB]); // VCVTTPS2DQ XMM1, XMM3
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7E, 0x88, 0x5B, 0x08]); // VCVTTPS2DQ XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::XMM1, None, Some(MergeMode::Zero)), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0x98, 0x5B, 0x08]); // VCVTTPS2DQ XMM1, DWORD PTR [EAX]{1to4}
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x7E, 0xA8, 0x5B, 0xCB]); // VCVTTPS2DQ YMM1, YMM3
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7E, 0xA8, 0x5B, 0x08]); // VCVTTPS2DQ YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0xB8, 0x5B, 0x08]); // VCVTTPS2DQ YMM1, DWORD PTR [EAX]{1to8}
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x7E, 0xC8, 0x5B, 0xCB]); // VCVTTPS2DQ ZMM1, ZMM3
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7E, 0xC8, 0x5B, 0x08]); // VCVTTPS2DQ ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VCVTTPS2DQ, Operand::AVXDestination(Reg::ZMM1, None, Some(MergeMode::Zero)), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0xD8, 0x5B, 0x08]); // VCVTTPS2DQ ZMM1, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_cvttps2pi() {
    encode32_helper2(Mnemonic::CVTTPS2PI, Operand::Direct(Reg::MM1), Operand::Direct(Reg::XMM0), &vec![0x0F, 0x2C, 0xC8]); // CVTTPS2PI 0x1, XMM0
    encode32_helper2(Mnemonic::CVTTPS2PI, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x2C, 0x08]); // CVTTPS2PI 0x1, QWORD PTR [EAX]
}

#[test]
fn instr_cvttsd2si() {
    encode32_helper2(Mnemonic::CVTTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xF2, 0x0F, 0x2C, 0xC1]); // CVTTSD2SI EAX, XMM1
    encode32_helper2(Mnemonic::CVTTSD2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x2C, 0x03]); // CVTTSD2SI EAX, QWORD PTR [EBX]
    encode32_helper2(Mnemonic::VCVTTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFB, 0x2C, 0xC1]); // VCVTTSD2SI EAX, XMM1
    encode32_helper2(Mnemonic::VCVTTSD2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Qword), None), &vec![0xC5, 0xFB, 0x2C, 0x03]); // VCVTTSD2SI EAX, QWORD PTR [EBX]
    encode32_helper2_flags(Mnemonic::VCVTTSD2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7F, 0x18, 0x2C, 0xC1]); // VCVTTSD2SI EAX, XMM1, {SAE}
}

#[test]
fn instr_cvttss2si() {
    encode32_helper2(Mnemonic::CVTTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xF3, 0x0F, 0x2C, 0xC1]); // CVTTSS2SI EAX, XMM1
    encode32_helper2(Mnemonic::CVTTSS2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x2C, 0x03]); // CVTTSS2SI EAX, DWORD PTR [EBX]
    encode32_helper2(Mnemonic::VCVTTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFA, 0x2C, 0xC1]); // VCVTTSS2SI EAX, XMM1
    encode32_helper2(Mnemonic::VCVTTSS2SI, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Dword), None), &vec![0xC5, 0xFA, 0x2C, 0x03]); // VCVTTSS2SI EAX, DWORD PTR [EBX]
    encode32_helper2_flags(Mnemonic::VCVTTSS2SI, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), InstructionFlags {
        sae: true,
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x7E, 0x18, 0x2C, 0xC1]); // VCVTTSS2SI EAX, XMM1, {SAE}
}

#[test]
fn instr_cwd() {
    encode32_helper0(Mnemonic::CWD,  &vec![0x66, 0x99]); // CWD
}

#[test]
fn instr_cdq() {
    encode32_helper0(Mnemonic::CDQ,  &vec![0x99]); // CDQ
}

#[test]
fn instr_daa() {
    encode32_helper0(Mnemonic::DAA,  &vec![0x27]); // DAA
}

#[test]
fn instr_das() {
    encode32_helper0(Mnemonic::DAS,  &vec![0x2F]); // DAS
}

#[test]
fn instr_dec() {
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xFE, 0x08]); // DEC BYTE PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xFF, 0x08]); // DEC WORD PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xFF, 0x08]); // DEC DWORD PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Direct(Reg::BX), &vec![0x66, 0x4B]); // DEC BX
    encode32_helper1(Mnemonic::DEC, Operand::Direct(Reg::EBX), &vec![0x4B]); // DEC EBX
}


#[test]
fn instr_div() {
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::BL), &vec![0xF6, 0xF3]); // DIV BL
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::BX), &vec![0x66, 0xF7, 0xF3]); // DIV BX
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::EBX), &vec![0xF7, 0xF3]); // DIV EBX
}

#[test]
fn instr_divpd() {
    encode32_helper2(Mnemonic::DIVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x5E, 0xCA]); // DIVPD XMM1, XMM2
    encode32_helper2(Mnemonic::DIVPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x5E, 0x08]); // DIVPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x5E, 0xCB]); // VDIVPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x5E, 0x08]); // VDIVPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x5E, 0xCB]); // VDIVPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VDIVPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x5E, 0x08]); // VDIVPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xED, 0x09, 0x5E, 0xCB]); // VDIVPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x09, 0x5E, 0x08]); // VDIVPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x19, 0x5E, 0x08]); // VDIVPD XMM1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0x29, 0x5E, 0xCB]); // VDIVPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0x29, 0x5E, 0x08]); // VDIVPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x39, 0x5E, 0x08]); // VDIVPD YMM1, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0x49, 0x5E, 0xCB]); // VDIVPD ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x49, 0x5E, 0x08]); // VDIVPD ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x59, 0x5E, 0x08]); // VDIVPD ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
    encode32_helper3_flags(Mnemonic::VDIVPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Up),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xED, 0x59, 0x5E, 0xCB]); // VDIVPD ZMM1, ZMM2, ZMM3
}

#[test]
fn instr_divps() {
    encode32_helper2(Mnemonic::DIVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x5E, 0xCA]); // DIVPS XMM1, XMM2
    encode32_helper2(Mnemonic::DIVPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x5E, 0x08]); // DIVPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x5E, 0xCB]); // VDIVPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x5E, 0x08]); // VDIVPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x5E, 0xCB]); // VDIVPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VDIVPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x5E, 0x08]); // VDIVPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6C, 0x09, 0x5E, 0xCB]); // VDIVPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6C, 0x09, 0x5E, 0x08]); // VDIVPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x19, 0x5E, 0x08]); // VDIVPS XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6C, 0x29, 0x5E, 0xCB]); // VDIVPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6C, 0x29, 0x5E, 0x08]); // VDIVPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x39, 0x5E, 0x08]); // VDIVPS YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6C, 0x49, 0x5E, 0xCB]); // VDIVPS ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6C, 0x49, 0x5E, 0x08]); // VDIVPS ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x59, 0x5E, 0x08]); // VDIVPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper3_flags(Mnemonic::VDIVPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Up),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x6C, 0x59, 0x5E, 0xCB]); // VDIVPD ZMM1, ZMM2, ZMM3, {RU-SAE}
}

#[test]
fn instr_divsd() {
    encode32_helper2(Mnemonic::DIVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x5E, 0xCA]); // DIVSD XMM1, XMM2
    encode32_helper2(Mnemonic::DIVSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x5E, 0x08]); // DIVSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x5E, 0xCB]); // VDIVSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x5E, 0x08]); // VDIVSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xEF, 0x09, 0x5E, 0x08]); // VDIVSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3_flags(Mnemonic::VDIVSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Zero),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0xEF, 0x79, 0x5E, 0xCB]); // VDIVSD XMM1, XMM2, XMM3, {RZ-SAE}
}

#[test]
fn instr_divss() {
    encode32_helper2(Mnemonic::DIVSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x5E, 0xCA]); // DIVSS XMM1, XMM2
    encode32_helper2(Mnemonic::DIVSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x5E, 0x08]); // DIVSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x5E, 0xCB]); // VDIVSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VDIVSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x5E, 0x08]); // VDIVSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VDIVSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6E, 0x09, 0x5E, 0x08]); // VDIVSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3_flags(Mnemonic::VDIVSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), InstructionFlags {
        rounding_mode: Some(RoundingMode::Zero),
        ..Default::default()
    }, &vec![0x62, 0xF1, 0x6E, 0x79, 0x5E, 0xCB]); // VDIVSD XMM1, XMM2, XMM3, {RZ-SAE}
}

#[test]
fn instr_dppd() {
    encode32_helper3(Mnemonic::DPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x41, 0xCA, 0x12]); // DPPD XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::DPPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x41, 0x08, 0x12]); // DPPD XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VDPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x41, 0xCB, 0x12]); // VDPPD XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VDPPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x41, 0x08, 0x12]); // VDPPD XMM1, XMM2, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_dpps() {
    encode32_helper3(Mnemonic::DPPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x40, 0xCA, 0x12]); // DPPS XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::DPPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x40, 0x08, 0x12]); // DPPS XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VDPPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x40, 0xCB, 0x12]); // VDPPS XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VDPPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x40, 0x08, 0x12]); // VDPPS XMM1, XMM2, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VDPPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x40, 0xCB, 0x12]); // VDPPS YMM1, YMM2, YMM3, 0x12
    encode32_helper4(Mnemonic::VDPPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x40, 0x08, 0x12]); // VDPPS YMM1, YMM2, YMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_emms() {
    encode32_helper0(Mnemonic::EMMS,  &vec![0x0F, 0x77]); // EMMS
}

#[test]
fn instr_enter() {
    encode32_helper2(Mnemonic::ENTER, Operand::Literal16(0x1234), Operand::Literal8(0x56), &vec![0xC8, 0x34, 0x12, 0x56]); // ENTER 0x1234, 0x56
}

#[test]
fn instr_extractps() {
    encode32_helper3(Mnemonic::EXTRACTPS, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x17, 0xC8, 0x12]); // EXTRACTPS EAX, XMM1, 0x12
    encode32_helper3(Mnemonic::EXTRACTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM1), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x17, 0x08, 0x12]); // EXTRACTPS DWORD PTR [EAX], XMM1, 0x12
    encode32_helper3(Mnemonic::VEXTRACTPS, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x17, 0xC8, 0x12]); // VEXTRACTPS EAX, XMM1, 0x12
    encode32_helper3(Mnemonic::VEXTRACTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM1), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x17, 0x08, 0x12]); // VEXTRACTPS DWORD PTR [EAX], XMM1, 0x12
}

#[test]
fn instr_f2xm1() {
    encode32_helper0(Mnemonic::F2XM1,  &vec![0xD9, 0xF0]); // F2XM1
}

#[test]
fn instr_fabs() {
    encode32_helper0(Mnemonic::FABS,  &vec![0xD9, 0xE1]); // FABS
}

#[test]
fn instr_fadd() {
    encode32_helper1(Mnemonic::FADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x00]); // FADD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x00]); // FADD QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FADD, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xC2]); // FADD ST, ST(2)
    encode32_helper2(Mnemonic::FADD, Operand::Direct(Reg::ST4), Operand::Direct(Reg::ST), &vec![0xDC, 0xC4]); // FADD ST(4), ST
    encode32_helper2(Mnemonic::FADDP, Operand::Direct(Reg::ST3), Operand::Direct(Reg::ST), &vec![0xDE, 0xC3]); // FADDP ST(3), ST
    encode32_helper2(Mnemonic::FADDP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xC1]); // FADDP ST(1), ST
    encode32_helper1(Mnemonic::FIADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x00]); // FIADD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x00]); // FIADD WORD PTR [EAX]
}

#[test]
fn instr_fbld() {
    encode32_helper1(Mnemonic::FBLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDF, 0x20]); // FBLD TBYTE PTR [EAX]
}

#[test]
fn instr_fbstp() {
    encode32_helper1(Mnemonic::FBSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDF, 0x30]); // FBSTP TBYTE PTR [EAX]
}

#[test]
fn instr_fclex() {
    encode32_helper0(Mnemonic::FCLEX,  &vec![0x9B, 0xDB, 0xE2]); // FCLEX 
}

#[test]
fn instr_fnclex() {
    encode32_helper0(Mnemonic::FNCLEX,  &vec![0xDB, 0xE2]); // FNCLEX 
}

#[test]
fn instr_fcmov() {
    encode32_helper2(Mnemonic::FCMOVB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xDA, 0xC1]); // FCMOVB ST, ST(1)
    encode32_helper2(Mnemonic::FCMOVE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xDA, 0xCA]); // FCMOVE ST, ST(2)
    encode32_helper2(Mnemonic::FCMOVBE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST3), &vec![0xDA, 0xD3]); // FCMOVBE ST, ST(3)
    encode32_helper2(Mnemonic::FCMOVU, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST4), &vec![0xDA, 0xDC]); // FCMOVU ST, ST(4)
    encode32_helper2(Mnemonic::FCMOVNB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST5), &vec![0xDB, 0xC5]); // FCMOVNB ST, ST(5)
    encode32_helper2(Mnemonic::FCMOVNE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST6), &vec![0xDB, 0xCE]); // FCMOVNE ST, ST(6)
    encode32_helper2(Mnemonic::FCMOVNBE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST7), &vec![0xDB, 0xD7]); // FCMOVNBE ST, ST(7)
    encode32_helper2(Mnemonic::FCMOVNU, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xDB, 0xD9]); // FCMOVNU ST, ST(1)
}

#[test]
fn instr_fcom() {
    encode32_helper1(Mnemonic::FCOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x10]); // FCOM DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x10]); // FCOM QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOM, Operand::Direct(Reg::ST2), &vec![0xD8, 0xD2]); // FCOM ST(2)
    encode32_helper0(Mnemonic::FCOM, &vec![0xD8, 0xD1]); // FCOM
    encode32_helper1(Mnemonic::FCOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x18]); // FCOMP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x18]); // FCOMP QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOMP, Operand::Direct(Reg::ST3), &vec![0xD8, 0xDB]); // FCOMP ST(3)
    encode32_helper1(Mnemonic::FCOMP, Operand::Direct(Reg::ST1), &vec![0xD8, 0xD9]); // FCOMP ST(1)
    encode32_helper0(Mnemonic::FCOMP, &vec![0xD8, 0xD9]); // FCOMP
    encode32_helper0(Mnemonic::FCOMPP, &vec![0xDE, 0xD9]); // FCOMPP 
}

#[test]
fn instr_fcomi() {
    encode32_helper2(Mnemonic::FCOMI, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST4), &vec![0xDB, 0xF4]); // FCOMI ST, ST(4)
    encode32_helper2(Mnemonic::FCOMIP, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST5), &vec![0xDF, 0xF5]); // FCOMIP ST, ST(5)
    encode32_helper2(Mnemonic::FUCOMI, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST6), &vec![0xDB, 0xEE]); // FUCOMI ST, ST(6)
    encode32_helper2(Mnemonic::FUCOMIP, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST7), &vec![0xDF, 0xEF]); // FUCOMIP ST, ST(7)
}

#[test]
fn instr_fcos() {
    encode32_helper0(Mnemonic::FCOS, &vec![0xD9, 0xFF]); // FCOS 
}

#[test]
fn instr_fdecstp() {
    encode32_helper0(Mnemonic::FDECSTP,  &vec![0xD9, 0xF6]); // FDECSTP 
}

#[test]
fn instr_fdiv() {
    // TODO Should probably auto-generate reverse forms as appropriate
    encode32_helper1(Mnemonic::FDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x30]); // FDIV DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x30]); // FDIV QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIV, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xF2]); // FDIV ST, ST(2)
    encode32_helper1(Mnemonic::FIDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x30]); // FIDIV DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x30]); // FIDIV WORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xF9]); // FDIVP ST(1), ST
}


#[test]
fn instr_fdivr() {
    encode32_helper1(Mnemonic::FDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x38]); // FDIVR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x38]); // FDIVR QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xD8, 0xF9]); // FDIVR ST, ST(1)
    encode32_helper2(Mnemonic::FDIVR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xFA]); // FDIVR ST, ST(2)
    encode32_helper1(Mnemonic::FIDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x38]); // FIDIVR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x38]); // FIDIVR WORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVRP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xF1]); // FDIVRP ST(1), ST
}

#[test]
fn instr_ffree() {
    encode32_helper1(Mnemonic::FFREE, Operand::Direct(Reg::ST4), &vec![0xDD, 0xC4]); // FFREE ST(4)
}

#[test]
fn instr_ficom() {
    encode32_helper1(Mnemonic::FICOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x10]); // FICOM WORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x10]); // FICOM DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x18]); // FICOMP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x18]); // FICOMP DWORD PTR [EAX]
}

#[test]
fn instr_fild() {
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x00]); // FILD WORD PTR [EAX]
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x00]); // FILD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDF, 0x28]); // FILD QWORD PTR [EAX]
}

#[test]
fn instr_fincstp() {
    encode32_helper0(Mnemonic::FINCSTP,  &vec![0xD9, 0xF7]); // FINCSTP 
}

#[test]
fn instr_finit() {
    encode32_helper0(Mnemonic::FINIT,  &vec![0x9B, 0xDB, 0xE3]); // FINIT 
    encode32_helper0(Mnemonic::FNINIT,  &vec![0xDB, 0xE3]); // FNINIT 
}

#[test]
fn instr_fist() {
    encode32_helper1(Mnemonic::FIST, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x10]); // FIST WORD PTR [EAX]
    encode32_helper1(Mnemonic::FIST, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x10]); // FIST DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x18]); // FISTP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x18]); // FISTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDF, 0x38]); // FISTP QWORD PTR [EAX]
}

#[test]
fn instr_fisttp() {
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x08]); // FISTTP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x08]); // FISTTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x08]); // FISTTP QWORD PTR [EAX]
}

#[test]
fn instr_fld() {
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x00]); // FLD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x00]); // FLD QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDB, 0x28]); // FLD TBYTE PTR [EAX]
}

#[test]
fn instr_fld_constant() {
    encode32_helper0(Mnemonic::FLD1,  &vec![0xD9, 0xE8]); // FLD1 
    encode32_helper0(Mnemonic::FLDL2T,  &vec![0xD9, 0xE9]); // FLDL2T 
    encode32_helper0(Mnemonic::FLDL2E,  &vec![0xD9, 0xEA]); // FLDL2E 
    encode32_helper0(Mnemonic::FLDPI,  &vec![0xD9, 0xEB]); // FLDPI 
    encode32_helper0(Mnemonic::FLDLG2,  &vec![0xD9, 0xEC]); // FLDLG2 
    encode32_helper0(Mnemonic::FLDLN2,  &vec![0xD9, 0xED]); // FLDLN2 
    encode32_helper0(Mnemonic::FLDZ,  &vec![0xD9, 0xEE]); // FLDZ 
}

#[test]
fn instr_fldcw() {
    encode32_helper1(Mnemonic::FLDCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xD9, 0x28]); // FLDCW WORD PTR [EAX]
}

#[test]
fn instr_fldenv() {
    encode32_helper1(Mnemonic::FLDENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xD9, 0x20]);
}

#[test]
fn instr_fmul() {
    encode32_helper1(Mnemonic::FMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x08]); // FMUL DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x08]); // FMUL QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FMUL, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xCA]); // FMUL ST, ST(2)
    encode32_helper2(Mnemonic::FMUL, Operand::Direct(Reg::ST2), Operand::Direct(Reg::ST), &vec![0xDC, 0xCA]); // FMUL ST(2), ST
    encode32_helper2(Mnemonic::FMULP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xC9]); // FMULP ST(1), ST
    encode32_helper1(Mnemonic::FIMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x08]); // FIMUL DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x08]); // FIMUL WORD PTR [EAX]
}

#[test]
fn instr_fnop() {
    encode32_helper0(Mnemonic::FNOP,  &vec![0xD9, 0xD0]); // FNOP 
}

#[test]
fn instr_fpatan() {
    encode32_helper0(Mnemonic::FPATAN,  &vec![0xD9, 0xF3]); // FPATAN 
}

#[test]
fn instr_fprem() {
    encode32_helper0(Mnemonic::FPREM,  &vec![0xD9, 0xF8]); // FPREM 
}

#[test]
fn instr_fptan() {
    encode32_helper0(Mnemonic::FPTAN,  &vec![0xD9, 0xF2]); // FPTAN 
}

#[test]
fn instr_frndint() {
    encode32_helper0(Mnemonic::FRNDINT,  &vec![0xD9, 0xFC]); // FRNDINT 
}

#[test]
fn instr_frstor() {
    encode32_helper1(Mnemonic::FRSTOR, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xDD, 0x20]); // FRSTOR [EAX]
}

#[test]
fn instr_fsave() {
    encode32_helper1(Mnemonic::FSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x9B, 0xDD, 0x30]); // FSAVE [EAX]
    encode32_helper1(Mnemonic::FNSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xDD, 0x30]); // FNSAVE [EAX]
}

#[test]
fn instr_fscale() {
    encode32_helper0(Mnemonic::FSCALE,  &vec![0xD9, 0xFD]); // FSCALE 
}

#[test]
fn instr_fsin() {
    encode32_helper0(Mnemonic::FSIN,  &vec![0xD9, 0xFE]); // FSIN 
}

#[test]
fn instr_fsincos() {
    encode32_helper0(Mnemonic::FSINCOS,  &vec![0xD9, 0xFB]); // FSINCOS 
}

#[test]
fn instr_fsqrt() {
    encode32_helper0(Mnemonic::FSQRT,  &vec![0xD9, 0xFA]); // FSQRT 
}

#[test]
fn instr_fst() {
    encode32_helper1(Mnemonic::FST, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x10]); // FST DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FST, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x10]); // FST QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FST, Operand::Direct(Reg::ST3), &vec![0xDD, 0xD3]); // FST ST(3)
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x18]); // FSTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x18]); // FSTP QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDB, 0x38]); // FSTP TBYTE PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Direct(Reg::ST4), &vec![0xDD, 0xDC]); // FSTP ST(4)
}

#[test]
fn instr_fstcw() {
    encode32_helper1(Mnemonic::FSTCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x9B, 0xD9, 0x38]); // FSTCW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FNSTCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xD9, 0x38]); // FNSTCW WORD PTR [EAX]
}

#[test]
fn instr_fstenv() {
    encode32_helper1(Mnemonic::FSTENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x9B, 0xD9, 0x30]); // FSTENV [EAX]
    encode32_helper1(Mnemonic::FNSTENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xD9, 0x30]); // FNSTENV [EAX]
}

#[test]
fn instr_fstsw() {
    encode32_helper1(Mnemonic::FSTSW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x9B, 0xDD, 0x38]); // FSTSW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTSW, Operand::Direct(Reg::AX), &vec![0x9B, 0xDF, 0xE0]); // FSTSW AX
    encode32_helper1(Mnemonic::FNSTSW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDD, 0x38]); // FNSTSW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FNSTSW, Operand::Direct(Reg::AX), &vec![0xDF, 0xE0]); // FNSTSW AX
}

#[test]
fn instr_fsub() {
    encode32_helper1(Mnemonic::FSUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x20]); // FSUB DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x20]); // FSUB QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FSUB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST3), &vec![0xD8, 0xE3]); // FSUB ST, ST(3)
    encode32_helper2(Mnemonic::FSUBP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xE9]); // FSUBP ST(1), ST
    encode32_helper1(Mnemonic::FISUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x20]); // FISUB DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x20]); // FISUB WORD PTR [EAX]
}

#[test]
fn instr_fsubr() {
    encode32_helper1(Mnemonic::FSUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x28]); // FSUBR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x28]); // FSUBR QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FSUBR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xEA]); // FSUBR ST, ST(2)
    encode32_helper2(Mnemonic::FSUBRP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xE1]); // FSUBRP ST(1), ST
    encode32_helper1(Mnemonic::FISUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x28]); // FISUBR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x28]); // FISUBR WORD PTR [EAX]
}

#[test]
fn instr_ftst() {
    encode32_helper0(Mnemonic::FTST,  &vec![0xD9, 0xE4]); // FTST 
}

#[test]
fn instr_fucom() {
    encode32_helper1(Mnemonic::FUCOM, Operand::Direct(Reg::ST2), &vec![0xDD, 0xE2]); // FUCOM ST(2)
    encode32_helper1(Mnemonic::FUCOM, Operand::Direct(Reg::ST1), &vec![0xDD, 0xE1]); // FUCOM ST(1)
    encode32_helper1(Mnemonic::FUCOMP, Operand::Direct(Reg::ST3), &vec![0xDD, 0xEB]); // FUCOMP ST(3)
    encode32_helper1(Mnemonic::FUCOMP, Operand::Direct(Reg::ST1), &vec![0xDD, 0xE9]); // FUCOMP ST(1)
    encode32_helper0(Mnemonic::FUCOMPP,  &vec![0xDA, 0xE9]); // FUCOMPP 
}

#[test]
fn instr_fxam() {
    encode32_helper0(Mnemonic::FXAM,  &vec![0xD9, 0xE5]); // FXAM 
}

#[test]
fn instr_fxch() {
    encode32_helper1(Mnemonic::FXCH, Operand::Direct(Reg::ST3), &vec![0xD9, 0xCB]); // FXCH ST(3)
    encode32_helper0(Mnemonic::FXCH, &vec![0xD9, 0xC9]); // FXCH
}

#[test]
fn instr_fxrstor() {
    encode32_helper1(Mnemonic::FXRSTOR, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x0F, 0xAE, 0x08]);
}

#[test]
fn instr_fxsave() {
    encode32_helper1(Mnemonic::FXSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x0F, 0xAE, 0x00]);
}

#[test]
fn instr_fxtract() {
    encode32_helper0(Mnemonic::FXTRACT,  &vec![0xD9, 0xF4]); // FXTRACT 
}

#[test]
fn instr_fyl2x() {
    encode32_helper0(Mnemonic::FYL2X,  &vec![0xD9, 0xF1]); // FYL2X 
}

#[test]
fn instr_fyl2xp1() {
    encode32_helper0(Mnemonic::FYL2XP1,  &vec![0xD9, 0xF9]); // FYL2XP1 
}

#[test]
fn instr_haddpd() {
    encode32_helper2(Mnemonic::HADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x7C, 0xCA]); // HADDPD XMM1, XMM2
    encode32_helper2(Mnemonic::HADDPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x7C, 0x08]); // HADDPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x7C, 0xCB]); // VHADDPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VHADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x7C, 0x08]); // VHADDPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x7C, 0xCB]); // VHADDPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VHADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x7C, 0x08]); // VHADDPD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_haddps() {
    encode32_helper2(Mnemonic::HADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x7C, 0xCA]); // HADDPS XMM1, XMM2
    encode32_helper2(Mnemonic::HADDPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF2, 0x0F, 0x7C, 0x08]); // HADDPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x7C, 0xCB]); // VHADDPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VHADDPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xEB, 0x7C, 0x08]); // VHADDPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHADDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEF, 0x7C, 0xCB]); // VHADDPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VHADDPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEF, 0x7C, 0x08]); // VHADDPS YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_hlt() {
    encode32_helper0(Mnemonic::HLT,  &vec![0xF4]); // HLT 
}

#[test]
fn instr_hsubpd() {
    encode32_helper2(Mnemonic::HSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x7D, 0xCA]); // HSUBPD XMM1, XMM2
    encode32_helper2(Mnemonic::HSUBPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x7D, 0x08]); // HSUBPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x7D, 0xCB]); // VHSUBPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VHSUBPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x7D, 0x08]); // VHSUBPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHSUBPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x7D, 0xCB]); // VHSUBPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VHSUBPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x7D, 0x08]); // VHSUBPD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_hsubps() {
    encode32_helper2(Mnemonic::HSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x7D, 0xCA]); // HSUBPS XMM1, XMM2
    encode32_helper2(Mnemonic::HSUBPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF2, 0x0F, 0x7D, 0x08]); // HSUBPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x7D, 0xCB]); // VHSUBPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VHSUBPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xEB, 0x7D, 0x08]); // VHSUBPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VHSUBPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEF, 0x7D, 0xCB]); // VHSUBPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VHSUBPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEF, 0x7D, 0x08]); // VHSUBPS YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_idiv() {
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xF6, 0x38]); // IDIV BYTE PTR [EAX]
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF7, 0x38]); // IDIV WORD PTR [EAX]
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF7, 0x38]); // IDIV DWORD PTR [EAX]
}

#[test]
fn instr_imul() {
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xF6, 0x28]); // IMUL BYTE PTR [EAX]
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF7, 0x28]); // IMUL WORD PTR [EAX]
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF7, 0x28]); // IMUL DWORD PTR [EAX]
    encode32_helper2(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0xAF, 0x18]); // IMUL BX, WORD PTR [EAX]
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x6B, 0x18, 0x12]); // IMUL BX, WORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x6B, 0x18, 0x12]); // IMUL EBX, DWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x69, 0x18, 0x34, 0x12]); // IMUL BX, WORD PTR [EAX], 0x1234
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x69, 0x18, 0x78, 0x56, 0x34, 0x12]); // IMUL EBX, DWORD PTR [EAX], 0x12345678
}

#[test]
fn instr_in() {
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0xE4, 0x12]); // IN AL, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AX), Operand::Literal8(0x12), &vec![0x66, 0xE5, 0x12]); // IN AX, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::EAX), Operand::Literal8(0x12), &vec![0xE5, 0x12]); // IN EAX, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AL), Operand::Direct(Reg::DX), &vec![0xEC]); // IN AL, DX
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AX), Operand::Direct(Reg::DX), &vec![0x66, 0xED]); // IN AX, DX
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::EAX), Operand::Direct(Reg::DX), &vec![0xED]); // IN EAX, DX
}

#[test]
fn instr_inc() {
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xFE, 0x00]); // INC BYTE PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xFF, 0x00]); // INC WORD PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xFF, 0x00]); // INC DWORD PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Direct(Reg::BX), &vec![0x66, 0x43]); // INC BX
    encode32_helper1(Mnemonic::INC, Operand::Direct(Reg::EBX), &vec![0x43]); // INC EBX
}

#[test]
fn instr_ins() {
    // TODO Could have these instructions have an optional DX operand, as shown in Intel docs
    encode32_helper0(Mnemonic::INSB, &vec![0x6C]);
    encode32_helper0(Mnemonic::INSW, &vec![0x66, 0x6D]);
    encode32_helper0(Mnemonic::INSD, &vec![0x6D]);
    encode32_helper1(Mnemonic::INS, Operand::Indirect(Reg::EDI, Some(OperandSize::Byte), Some(SegmentReg::ES)), &vec![0x6C]);
    encode32_helper1(Mnemonic::INS, Operand::Indirect(Reg::EDI, Some(OperandSize::Word), Some(SegmentReg::ES)), &vec![0x66, 0x6D]);
    encode32_helper1(Mnemonic::INS, Operand::Indirect(Reg::EDI, Some(OperandSize::Dword), Some(SegmentReg::ES)), &vec![0x6D]);
    // TODO Add forms of instruction that infer operand size based on memory argument (see Intel
    // docs)
}

#[test]
fn instr_insertps() {
    encode32_helper3(Mnemonic::INSERTPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x21, 0xCA, 0x12]); // INSERTPS XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::INSERTPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x21, 0x08, 0x12]); // INSERTPS XMM1, DWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VINSERTPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x21, 0xCB, 0x12]); // VINSERTPS XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VINSERTPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x21, 0x08, 0x12]); // VINSERTPS XMM1, XMM2, DWORD PTR [EAX], 0x12
    // NOTE: There's also an EVEX form specified in Intel docs. Doens't seem to be able to do
    // anything the VEX form can't do?
}

#[test]
fn instr_int() {
    encode32_helper1(Mnemonic::INT, Operand::Literal8(0x3), &vec![0xCC]); // INT 3 
    encode32_helper1(Mnemonic::INT, Operand::Literal8(0x5), &vec![0xCD, 0x05]); // INT 0x5
    encode32_helper0(Mnemonic::INTO, &vec![0xCE]); // INTO 
}

#[test]
fn instr_invd() {
    encode32_helper0(Mnemonic::INVD, &vec![0x0F, 0x08]); // INVD 
}

#[test]
fn instr_invlpg() {
   encode32_helper1( Mnemonic::INVLPG, Operand::Indirect(Reg::EAX, None, None), &vec![0x0f, 0x01, 0x38]); // INVLPG [EAX]
}

#[test]
fn instr_invpcid() {
    encode32_helper2(Mnemonic::INVPCID, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x82, 0x18]); // INVPCID EBX, XMMWORD PTR [EAX]
}

#[test]
fn instr_iret() {
    encode32_helper0(Mnemonic::IRET, &vec![0xCF]); // IRET 
    encode32_helper0(Mnemonic::IRETD, &vec![0xCF]); // IRETD
}

#[test]
fn instr_jcc() {
    fn jcc8_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        encode32_helper1(mnemonic, Operand::Literal8(0x12), &vec![primary_opcode, 0x12]);
    }

    fn jcc8_test_helper_aliased(mnemonics: &[Mnemonic], primary_opcode: u8) {
        test_aliased(mnemonics, |m| jcc8_test_helper(m, primary_opcode) );
    }    

    // OF
    jcc8_test_helper(Mnemonic::JO, 0x70);

    // !OF
    jcc8_test_helper(Mnemonic::JNO, 0x71);

    // CF
    jcc8_test_helper_aliased(&[Mnemonic::JB, Mnemonic::JNAE, Mnemonic::JC], 0x72);

    // !CF
    jcc8_test_helper_aliased(&[Mnemonic::JNB, Mnemonic::JAE, Mnemonic::JNC], 0x73);

    // ZF
    jcc8_test_helper_aliased(&[Mnemonic::JZ, Mnemonic::JE], 0x74);

    // !ZF
    jcc8_test_helper_aliased(&[Mnemonic::JNZ, Mnemonic::JNE], 0x75);

    // CF+ZF
    jcc8_test_helper_aliased(&[Mnemonic::JBE, Mnemonic::JNA], 0x76);

    // !CF*!ZF
    jcc8_test_helper_aliased(&[Mnemonic::JNBE, Mnemonic::JA], 0x77);

    // SF
    jcc8_test_helper(Mnemonic::JS, 0x78);

    // !SF
    jcc8_test_helper(Mnemonic::JNS, 0x79);

    // PF
    jcc8_test_helper_aliased(&[Mnemonic::JP, Mnemonic::JPE], 0x7A);
    
    // !PF
    jcc8_test_helper_aliased(&[Mnemonic::JNP, Mnemonic::JPO], 0x7B);

    // SF!=0F
    jcc8_test_helper_aliased(&[Mnemonic::JL, Mnemonic::JNGE], 0x7C);
    
    // SF=OF
    jcc8_test_helper_aliased(&[Mnemonic::JNL, Mnemonic::JGE], 0x7D);

    // ZF+(SF!=OF)
    jcc8_test_helper_aliased(&[Mnemonic::JLE, Mnemonic::JNG], 0x7E);

    // !ZF*(SF=OF)
    jcc8_test_helper_aliased(&[Mnemonic::JNLE, Mnemonic::JG], 0x7F);

    // CX == 0
    encode32_helper1(Mnemonic::JCXZ, Operand::Literal8(0x12), &vec![0x67, 0xE3, 0x12]);

    // ECX == 0
    encode32_helper1(Mnemonic::JECXZ, Operand::Literal8(0x12), &vec![0xE3, 0x12]);

    fn jcc16_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        encode32_helper1(mnemonic, Operand::Literal16(0x1234), &vec![0x66, 0x0F, primary_opcode, 0x34, 0x12]);
    }
    
    fn jcc32_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        encode32_helper1(mnemonic, Operand::Literal32(0x12345678), &vec![0x0F, primary_opcode, 0x78, 0x56, 0x34, 0x12]);
    }

    fn jcc_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        jcc16_test_helper(mnemonic, primary_opcode);
        jcc32_test_helper(mnemonic, primary_opcode);
    }

    fn jcc_test_helper_aliased(mnemonics: &[Mnemonic], primary_opcode: u8) {
        test_aliased(mnemonics, |m| jcc_test_helper(m, primary_opcode) );
    }

    // OF
    jcc_test_helper(Mnemonic::JO, 0x80);

    // !OF
    jcc_test_helper(Mnemonic::JNO, 0x81);

    // CF
    jcc_test_helper_aliased(&[Mnemonic::JB, Mnemonic::JNAE, Mnemonic::JC], 0x82);

    // !CF
    jcc_test_helper_aliased(&[Mnemonic::JNB, Mnemonic::JAE, Mnemonic::JNC], 0x83);

    // ZF
    jcc_test_helper_aliased(&[Mnemonic::JZ, Mnemonic::JE], 0x84);

    // !ZF
    jcc_test_helper_aliased(&[Mnemonic::JNZ, Mnemonic::JNE], 0x85);

    // CF+ZF
    jcc_test_helper_aliased(&[Mnemonic::JBE, Mnemonic::JNA], 0x86);

    // !CF*!ZF
    jcc_test_helper_aliased(&[Mnemonic::JNBE, Mnemonic::JA], 0x87);

    // SF
    jcc_test_helper(Mnemonic::JS, 0x88);

    // !SF
    jcc_test_helper(Mnemonic::JNS, 0x89);

    // PF
    jcc_test_helper_aliased(&[Mnemonic::JP, Mnemonic::JPE], 0x8A);
    
    // !PF
    jcc_test_helper_aliased(&[Mnemonic::JNP, Mnemonic::JPO], 0x8B);

    // SF!=0F
    jcc_test_helper_aliased(&[Mnemonic::JL, Mnemonic::JNGE], 0x8C);
    
    // SF=OF
    jcc_test_helper_aliased(&[Mnemonic::JNL, Mnemonic::JGE], 0x8D);

    // ZF+(SF!=OF)
    jcc_test_helper_aliased(&[Mnemonic::JLE, Mnemonic::JNG], 0x8E);

    // !ZF*(SF=OF)
    jcc_test_helper_aliased(&[Mnemonic::JNLE, Mnemonic::JG], 0x8F);
}

#[test]
fn instr_jmp() {
    encode32_helper1(Mnemonic::JMP, Operand::Literal8(0x12), &vec![0xEB, 0x12]); // JMP 0x12
    encode32_helper1(Mnemonic::JMP, Operand::Literal16(0x1234), &vec![0x66, 0xE9, 0x34, 0x12]); // JMP 0x1234
    encode32_helper1(Mnemonic::JMP, Operand::Literal32(0x12345678), &vec![0xE9, 0x78, 0x56, 0x34, 0x12]); // JMP 0x12345678
    encode32_helper1(Mnemonic::JMP, Operand::Direct(Reg::AX), &vec![0x66, 0xFF, 0xE0]); // JMP AX
    encode32_helper1(Mnemonic::JMP, Operand::Direct(Reg::EAX), &vec![0xFF, 0xE0]); // JMP EAX
    // TODO The JMPF forms should be aliased to JMP
    encode32_helper1(Mnemonic::JMPF, Operand::MemoryAndSegment16(0x1234, 0x5678), &vec![0x66, 0xEA, 0x78, 0x56, 0x34, 0x12]); // JMP 0x1234:0x5678
    encode32_helper1(Mnemonic::JMPF, Operand::MemoryAndSegment32(0x1234, 0x56789ABC), &vec![0xEA, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]); // JMP 0x1234:0x56789ABC
    encode32_helper1(Mnemonic::JMPF, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0xff, 0x28]); // JMP FWORD PTR [EAX]
    encode32_helper1(Mnemonic::JMPF, Operand::Indirect(Reg::EAX, Some(OperandSize::Fword), None), &vec![0xff, 0x28]); // JMP FWORD PTR [EAX]
}

#[test]
fn instr_kadd() {
    encode32_helper3(Mnemonic::KADDB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x4A, 0xC2]); // KADDB K0, K1, K2
    encode32_helper3(Mnemonic::KADDW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x4A, 0xC2]); // KADDW K0, K1, K2
    encode32_helper3(Mnemonic::KADDD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x4A, 0xC2]); // KADDD K0, K1, K2
    encode32_helper3(Mnemonic::KADDQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x4A, 0xC2]); // KADDQ K0, K1, K2
}

#[test]
fn instr_kand() {
    encode32_helper3(Mnemonic::KANDB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x41, 0xC2]); // KANDB K0, K1, K2
    encode32_helper3(Mnemonic::KANDW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x41, 0xC2]); // KANDW K0, K1, K2
    encode32_helper3(Mnemonic::KANDD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x41, 0xC2]); // KANDD K0, K1, K2
    encode32_helper3(Mnemonic::KANDQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x41, 0xC2]); // KANDQ K0, K1, K2
}

#[test]
fn instr_kandnw() {
    encode32_helper3(Mnemonic::KANDNB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x42, 0xC2]); // KANDNB K0, K1, K2
    encode32_helper3(Mnemonic::KANDNW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x42, 0xC2]); // KANDNW K0, K1, K2
    encode32_helper3(Mnemonic::KANDND, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x42, 0xC2]); // KANDND K0, K1, K2
    encode32_helper3(Mnemonic::KANDNQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x42, 0xC2]); // KANDNQ K0, K1, K2
}

#[test]
fn instr_kmov() {
   encode32_helper2(Mnemonic::KMOVB, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF9, 0x90, 0xCA]); // KMOVB K1, K2
   encode32_helper2(Mnemonic::KMOVB, Operand::Direct(Reg::K1), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xC5, 0xF9, 0x90, 0x08]); // KMOVB K1, BYTE PTR [EAX]
   encode32_helper2(Mnemonic::KMOVW, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF8, 0x90, 0xCA]); // KMOVW K1, K2
   encode32_helper2(Mnemonic::KMOVW, Operand::Direct(Reg::K1), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xC5, 0xF8, 0x90, 0x08]); // KMOVW K1, WORD PTR [EAX]
   encode32_helper2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF9, 0x90, 0xCA]); // KMOVD K1, K2
   encode32_helper2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE1, 0xF9, 0x90, 0x08]); // KMOVD K1, DWORD PTR [EAX]
   encode32_helper2(Mnemonic::KMOVQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF8, 0x90, 0xCA]); // KMOVQ K1, K2
   encode32_helper2(Mnemonic::KMOVQ, Operand::Direct(Reg::K1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE1, 0xF8, 0x90, 0x08]); // KMOVQ K1, QWORD PTR [EAX]
   encode32_helper2(Mnemonic::KMOVB, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::K1), &vec![0xC5, 0xF9, 0x91, 0x08]); // KMOVB BYTE PTR [EAX], K1
   encode32_helper2(Mnemonic::KMOVW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::K1), &vec![0xC5, 0xF8, 0x91, 0x08]); // KMOVW WORD PTR [EAX], K1
   encode32_helper2(Mnemonic::KMOVD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF9, 0x91, 0x08]); // KMOVD DWORD PTR [EAX], K1
   encode32_helper2(Mnemonic::KMOVQ, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF8, 0x91, 0x08]); // KMOVQ QWORD PTR [EAX], K1
   encode32_helper2(Mnemonic::KMOVB, Operand::Direct(Reg::EAX), Operand::Direct(Reg::K1), &vec![0xC5, 0xF9, 0x93, 0xC1]); // KMOVB EAX, K1
   encode32_helper2(Mnemonic::KMOVW, Operand::Direct(Reg::EAX), Operand::Direct(Reg::K1), &vec![0xC5, 0xF8, 0x93, 0xC1]); // KMOVW EAX, K1
   encode32_helper2(Mnemonic::KMOVD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::K1), &vec![0xC5, 0xFB, 0x93, 0xC1]); // KMOVD EAX, K1
   encode32_helper2(Mnemonic::KMOVB, Operand::Direct(Reg::K1), Operand::Direct(Reg::EAX), &vec![0xC5, 0xF9, 0x92, 0xC8]); // KMOVB K1, EAX
   encode32_helper2(Mnemonic::KMOVW, Operand::Direct(Reg::K1), Operand::Direct(Reg::EAX), &vec![0xC5, 0xF8, 0x92, 0xC8]); // KMOVW K1, EAX
   encode32_helper2(Mnemonic::KMOVD, Operand::Direct(Reg::K1), Operand::Direct(Reg::EAX), &vec![0xC5, 0xFB, 0x92, 0xC8]); // KMOVD K1, EAX:
}

#[test]
fn instr_knot() {
    encode32_helper2(Mnemonic::KNOTB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF9, 0x44, 0xC1]); // KNOTB K0, K1
    encode32_helper2(Mnemonic::KNOTW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF8, 0x44, 0xC1]); // KNOTW K0, K1
    encode32_helper2(Mnemonic::KNOTD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF9, 0x44, 0xC1]); // KNOTD K0, K1
    encode32_helper2(Mnemonic::KNOTQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF8, 0x44, 0xC1]); // KNOTQ K0, K1
}

#[test]
fn instr_kor() {
    encode32_helper3(Mnemonic::KORB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x45, 0xC2]); // KORB K0, K1, K2
    encode32_helper3(Mnemonic::KORW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x45, 0xC2]); // KORW K0, K1, K2
    encode32_helper3(Mnemonic::KORD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x45, 0xC2]); // KORD K0, K1, K2
    encode32_helper3(Mnemonic::KORQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x45, 0xC2]); // KORQ K0, K1, K2
}

#[test]
fn instr_kortest() {
    encode32_helper2(Mnemonic::KORTESTB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF9, 0x98, 0xC1]); // KORTESTB K0, K1
    encode32_helper2(Mnemonic::KORTESTW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF8, 0x98, 0xC1]); // KORTESTW K0, K1
    encode32_helper2(Mnemonic::KORTESTD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF9, 0x98, 0xC1]); // KORTESTD K0, K1
    encode32_helper2(Mnemonic::KORTESTQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF8, 0x98, 0xC1]); // KORTESTQ K0, K1
}

#[test]
fn instr_kshiftl() {
    encode32_helper3(Mnemonic::KSHIFTLB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x32, 0xC1, 0x01]); // KSHIFTLB K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTLW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0xF9, 0x32, 0xC1, 0x01]); // KSHIFTLW K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTLD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x33, 0xC1, 0x01]); // KSHIFTLD K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTLQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0xF9, 0x33, 0xC1, 0x01]); // KSHIFTLQ K0, K1, 0x1
}

#[test]
fn instr_kshiftr() {
    encode32_helper3(Mnemonic::KSHIFTRB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x30, 0xC1, 0x01]); // KSHIFTRB K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTRW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0xF9, 0x30, 0xC1, 0x01]); // KSHIFTRW K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTRD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x31, 0xC1, 0x01]); // KSHIFTRD K0, K1, 0x1
    encode32_helper3(Mnemonic::KSHIFTRQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0xF9, 0x31, 0xC1, 0x01]); // KSHIFTRQ K0, K1, 0x1
}

#[test]
fn instr_ktest() {
    encode32_helper2(Mnemonic::KTESTB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF9, 0x99, 0xC1]); // KTESTB K0, K1
    encode32_helper2(Mnemonic::KTESTW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC5, 0xF8, 0x99, 0xC1]); // KTESTW K0, K1
    encode32_helper2(Mnemonic::KTESTD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF9, 0x99, 0xC1]); // KTESTD K0, K1
    encode32_helper2(Mnemonic::KTESTQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), &vec![0xC4, 0xE1, 0xF8, 0x99, 0xC1]); // KTESTQ K0, K1
}

#[test]
fn instr_kunpck() {
    encode32_helper3(Mnemonic::KUNPCKBW, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), Operand::Direct(Reg::K3), &vec![0xC5, 0xED, 0x4B, 0xCB]); // KUNPCKBW K1, K2, K3
    encode32_helper3(Mnemonic::KUNPCKWD, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), Operand::Direct(Reg::K3), &vec![0xC5, 0xEC, 0x4B, 0xCB]); // KUNPCKWD K1, K2, K3
    encode32_helper3(Mnemonic::KUNPCKDQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), Operand::Direct(Reg::K3), &vec![0xC4, 0xE1, 0xEC, 0x4B, 0xCB]); // KUNPCKDQ K1, K2, K3
}

#[test]
fn instr_kxnor() {
    encode32_helper3(Mnemonic::KXNORB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x46, 0xC2]); // KXNORB K0, K1, K2
    encode32_helper3(Mnemonic::KXNORW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x46, 0xC2]); // KXNORW K0, K1, K2
    encode32_helper3(Mnemonic::KXNORD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x46, 0xC2]); // KXNORD K0, K1, K2
    encode32_helper3(Mnemonic::KXNORQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x46, 0xC2]); // KXNORQ K0, K1, K2
}

#[test]
fn instr_kxor() {
    encode32_helper3(Mnemonic::KXORB, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF5, 0x47, 0xC2]); // KXORB K0, K1, K2
    encode32_helper3(Mnemonic::KXORW, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC5, 0xF4, 0x47, 0xC2]); // KXORW K0, K1, K2
    encode32_helper3(Mnemonic::KXORD, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF5, 0x47, 0xC2]); // KXORD K0, K1, K2
    encode32_helper3(Mnemonic::KXORQ, Operand::Direct(Reg::K0), Operand::Direct(Reg::K1), Operand::Direct(Reg::K2), &vec![0xC4, 0xE1, 0xF4, 0x47, 0xC2]); // KXORQ K0, K1, K2
}

#[test]
fn instr_lahf() {
    encode32_helper0(Mnemonic::LAHF,&vec![0x9F]); // LAHF 
}

#[test]
fn instr_lar() {
    encode32_helper2(Mnemonic::LAR, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0x02, 0x18]); // LAR BX, WORD PTR [EAX]
    // TODO This form shouldn't enforce operand size matches (LAR EBX, AX should encode)
    encode32_helper2(Mnemonic::LAR, Operand::Direct(Reg::EBX), Operand::Direct(Reg::EAX), &vec![0x0F, 0x02, 0xD8]); // LAR EBX, AX
}

#[test]
fn instr_lddqu() {
    encode32_helper2(Mnemonic::LDDQU, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF2, 0x0F, 0xF0, 0x08]); // LDDQU XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VLDDQU, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFB, 0xF0, 0x08]); // VLDDQU XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VLDDQU, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFF, 0xF0, 0x08]); // VLDDQU YMM1, YMMWORD PTR [EAX]
}

#[test]
fn instr_ldmxcsr() {
    encode32_helper1(Mnemonic::LDMXCSR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0F, 0xAE, 0x10]); // LDMXCSR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::VLDMXCSR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xF8, 0xAE, 0x10]); // VLDMXCSR DWORD PTR [EAX]
}

#[test]
fn instr_load_segreg() {
    fn test_load_segreg_helper(mnemonic: Mnemonic, is_two_byte_opcode: bool, primary_opcode: u8) {
        if !is_two_byte_opcode {
            encode32_helper2(mnemonic, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, primary_opcode, 0x00]);
            encode32_helper2(mnemonic, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Fword), None), &vec![primary_opcode, 0x00]);
        } else {
            encode32_helper2(mnemonic, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x0F, primary_opcode, 0x00]);
            encode32_helper2(mnemonic, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Fword), None), &vec![0x0F, primary_opcode, 0x00]);
        }
    }

    test_load_segreg_helper(Mnemonic::LDS, false, 0xC5); // LDS
    test_load_segreg_helper(Mnemonic::LSS, true, 0xB2); // LSS
    test_load_segreg_helper(Mnemonic::LES, false, 0xC4); // LES
    test_load_segreg_helper(Mnemonic::LFS, true, 0xB4); // LFS
    test_load_segreg_helper(Mnemonic::LGS, true, 0xB5); // LGS
}

#[test]
fn instr_lea() {
    encode32_helper2(Mnemonic::LEA, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, None, None), &vec![0x66, 0x8D, 0x00]); // LEA AX, [EAX]
    encode32_helper2(Mnemonic::LEA, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x8d, 0x00]); // LEA AX, [EAX]
}

#[test]
fn instr_leave() {
    encode32_helper0(Mnemonic::LEAVE, &vec![0xC9]); // LEAVE
    // TODO Add def for LEAVEW (16-bit form), or maybe allow instruction suffixes?
}

#[test]
fn instr_lfence() {
    encode32_helper0(Mnemonic::LFENCE, &vec![0x0F, 0xAE, 0xE8]); // LFENCE 
}

#[test]
fn instr_lgdt() {
    encode32_helper1(Mnemonic::LGDT, Operand::Indirect(Reg::EAX, None, None), &vec![0x0F, 0x01, 0x10]); // LGDT [EAX]
}

#[test]
fn instr_lidt() {
    encode32_helper1(Mnemonic::LIDT, Operand::Indirect(Reg::EAX, None, None), &vec![0x0F, 0x01, 0x18]); // LGDT [EAX]
}

#[test]
fn instr_lldt() {
    encode32_helper1(Mnemonic::LLDT, Operand::Direct(Reg::AX), &vec![0x0F, 0x00, 0xD0]); // LLDT AX
}

#[test]
fn instr_lmsw() {
    encode32_helper1(Mnemonic::LMSW, Operand::Direct(Reg::AX), &vec![0x0F, 0x01, 0xF0]); // LMSW AX
}

#[test]
fn instr_lods() {
    encode32_helper1(Mnemonic::LODS, Operand::Indirect(Reg::ESI, Some(OperandSize::Byte), Some(SegmentReg::DS)), &vec![0xAC]); // LODS BYTE PTR DS:[ESI]
    encode32_helper1(Mnemonic::LODS, Operand::Indirect(Reg::ESI, Some(OperandSize::Word), Some(SegmentReg::DS)), &vec![0x66, 0xAD]); // LODS WORD PTR DS:[ESI]
    encode32_helper1(Mnemonic::LODS, Operand::Indirect(Reg::ESI, Some(OperandSize::Dword), Some(SegmentReg::DS)), &vec![0xAD]); // LODS DWORD PTR DS:[ESI]
    encode32_helper0(Mnemonic::LODSB, &vec![0xAC]); // LODSB
    encode32_helper0(Mnemonic::LODSW, &vec![0x66, 0xAD]); // LODSW
    encode32_helper0(Mnemonic::LODSD, &vec![0xAD]); // LODSD
}

#[test]
fn instr_loop() {
    encode32_helper1(Mnemonic::LOOP, Operand::Literal8(0x12), &vec![0xE2, 0x12]); // LOOP 0x12
    encode32_helper1(Mnemonic::LOOPE, Operand::Literal8(0x12), &vec![0xE1, 0x12]); // LOOP 0x12
    encode32_helper1(Mnemonic::LOOPNE, Operand::Literal8(0x12), &vec![0xE0, 0x12]); // LOOP 0x12
}

#[test]
fn instr_lsl() {
    // TODO Support mismatched operand sizes (see Intel docs)
    encode32_helper2(Mnemonic::LSL, Operand::Direct(Reg::AX), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0x03, 0xC3]); // LSL AX, BX
    encode32_helper2(Mnemonic::LSL, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, Some(OperandSize::Word), None), &vec![0x0F, 0x03, 0x03]); // LSL EAX, WORD PTR [EBX]
}

#[test]
fn instr_ltr() {
    encode32_helper1(Mnemonic::LTR, Operand::Direct(Reg::AX), &vec![0x0F, 0x00, 0xD8]); // LTR AX
}

#[test]
fn instr_lzcnt() {
    encode32_helper2(Mnemonic::LZCNT, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF3, 0x0F, 0xBD, 0x00]); // LZCNT AX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::LZCNT, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0xBD, 0x00]); // LZCNT EAX, DWORD PTR [EAX]
}
