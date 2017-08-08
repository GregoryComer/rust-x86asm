use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, InstructionFlags, RegScale};
use ::test::{encode32_helper0, encode32_helper1, encode32_helper2, encode32_helper2_flags, encode32_helper3, encode32_helper3_flags, encode32_helper4, test_aliased};

#[test]
fn instr_maskmovdqu() {
    encode32_helper2(Mnemonic::MASKMOVDQU, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xF7, 0xCA]); // MASKMOVDQU XMM1, XMM2
}

#[test]
fn instr_maxpd() {
    encode32_helper2(Mnemonic::MAXPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x5F, 0xCA]); // MAXPD XMM1, XMM2
    encode32_helper2(Mnemonic::MAXPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x5F, 0x08]); // MAXPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x5F, 0xCB]); // VMAXPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x5F, 0x08]); // VMAXPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x5F, 0x08]); // VMAXPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xED, 0x0A, 0x5F, 0xCB]); // VMAXPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXPD, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0xA8, 0x5F, 0x08]); // VMAXPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x58, 0x5F, 0x08]); // VMAXPD ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_maxps() {
    encode32_helper2(Mnemonic::MAXPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x5F, 0xCA]); // MAXPS XMM1, XMM2
    encode32_helper2(Mnemonic::MAXPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x5F, 0x08]); // MAXPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x5F, 0xCB]); // VMAXPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x5F, 0x08]); // VMAXPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x5F, 0x08]); // VMAXPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6C, 0x0A, 0x5F, 0xCB]); // VMAXPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXPS, Operand::AVXDestination(Reg::YMM1, None, Some(MergeMode::Zero)), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6C, 0xA8, 0x5F, 0x08]); // VMAXPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXPS, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x58, 0x5F, 0x08]); // VMAXPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_maxsd() {
    encode32_helper2(Mnemonic::MAXSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x5F, 0xCA]); // MAXSD XMM1, XMM2
    encode32_helper2(Mnemonic::MAXSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x5F, 0x08]); // MAXSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x5F, 0xCB]); // VMAXSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x5F, 0x08]); // VMAXSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xEF, 0x0A, 0x5F, 0xCB]); // VMAXSD XMM1, XMM2, XMM3
}

#[test]
fn instr_maxss() {
    encode32_helper2(Mnemonic::MAXSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x5F, 0xCA]); // MAXSS XMM1, XMM2
    encode32_helper2(Mnemonic::MAXSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x5F, 0x08]); // MAXSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x5F, 0xCB]); // VMAXSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMAXSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x5F, 0x08]); // VMAXSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMAXSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6E, 0x0A, 0x5F, 0xCB]); // VMAXSS XMM1, XMM2, XMM3
}

#[test]
fn instr_mfence() {
    encode32_helper0(Mnemonic::MFENCE,  &vec![0x0F, 0xAE, 0xF0]); // MFENCE 
}

#[test]
fn instr_minpd() {
    encode32_helper2(Mnemonic::MINPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x5D, 0xCA]); // MINPD XMM1, XMM2
    encode32_helper2(Mnemonic::MINPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x5D, 0x08]); // MINPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x5D, 0xCB]); // VMINPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMINPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x5D, 0xCB]); // VMINPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMINPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x59, 0x5D, 0x08]); // VMINPD ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_minps() {
    encode32_helper2(Mnemonic::MINPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x5D, 0xCA]); // MINPS XMM1, XMM2
    encode32_helper2(Mnemonic::MINPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x5D, 0x08]); // MINPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x5D, 0xCB]); // VMINPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMINPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x5D, 0xCB]); // VMINPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMINPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x59, 0x5D, 0x08]); // VMINPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_minsd() {
    encode32_helper2(Mnemonic::MINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x5D, 0xCA]); // MINSD XMM1, XMM2
    encode32_helper2(Mnemonic::MINSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x5D, 0x08]); // MINSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x5D, 0xCB]); // VMINSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x5D, 0x08]); // VMINSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xEF, 0x09, 0x5D, 0x08]); // VMINSD XMM1, XMM2, QWORD PTR [EAX]
}

#[test]
fn instr_minss() {
    encode32_helper2(Mnemonic::MINSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x5D, 0xCA]); // MINSS XMM1, XMM2
    encode32_helper2(Mnemonic::MINSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x5D, 0x08]); // MINSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x5D, 0xCB]); // VMINSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMINSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x5D, 0x08]); // VMINSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMINSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K1), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6E, 0x09, 0x5D, 0x08]); // VMINSS XMM1, XMM2, DWORD PTR [EAX]
}

#[test]
fn instr_monitor() {
    encode32_helper0(Mnemonic::MONITOR,  &vec![0x0F, 0x01, 0xC8]); // MONITOR 
}

#[test]
fn instr_mov() {
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x88, 0x18]); // MOV BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x89, 0x18]); // MOV WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x89, 0x18]); // MOV DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x8A, 0x18]); // MOV BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x8B, 0x18]); // MOV BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x8B, 0x18]); // MOV EBX, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::BX), Operand::Direct(Reg::SS), &vec![0x66, 0x8C, 0xD3]); // MOV BX, SS
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::SS), Operand::Direct(Reg::BX), &vec![0x8E, 0xD3]); // MOV SS, BX
    // TODO Test the next 3 instructions with dword-sized addresses (to test the non-addres size
    // prefix encoding)
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::AL), Operand::Offset(0x12, Some(OperandSize::Byte), Some(SegmentReg::ES)), &vec![0x26, 0x67, 0xA0, 0x12, 0x00]); // MOV AL, ES:0x12
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::AX), Operand::Offset(0x12, Some(OperandSize::Word), Some(SegmentReg::ES)), &vec![0x26, 0x67, 0x66, 0xA1, 0x12, 0x00]); // MOV AX, ES:0x12
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Offset(0x12, Some(OperandSize::Dword), Some(SegmentReg::ES)), &vec![0x26, 0x67, 0xA1, 0x12, 0x00]); // MOV EAX, ES:0x12
    encode32_helper2(Mnemonic::MOV, Operand::Offset(0x12, Some(OperandSize::Byte), Some(SegmentReg::ES)), Operand::Direct(Reg::AL), &vec![0x26, 0x67, 0xA2, 0x12, 0x00]); // MOV ES:0x12, AL
    encode32_helper2(Mnemonic::MOV, Operand::Offset(0x12, Some(OperandSize::Word), Some(SegmentReg::ES)), Operand::Direct(Reg::AX), &vec![0x26, 0x67, 0x66, 0xA3, 0x12, 0x00]); // MOV ES:0x12, AX
    encode32_helper2(Mnemonic::MOV, Operand::Offset(0x12, Some(OperandSize::Dword), Some(SegmentReg::ES)), Operand::Direct(Reg::EAX), &vec![0x26, 0x67, 0xA3, 0x12, 0x00]); // MOV ES:0x12, EAX
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0xB3, 0x12]); // MOV BL, 0x12
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0xBB, 0x34, 0x12]); // MOV BX, 0x1234
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(0x1234567), &vec![0xBB, 0x67, 0x45, 0x23, 0x01]); // MOV EBX, 0x1234567
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0xC6, 0x00, 0x12]); // MOV BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0xC7, 0x00, 0x34, 0x12]); // MOV WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::MOV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0xC7, 0x00, 0x78, 0x56, 0x34, 0x12]); // MOV DWORD PTR [EAX], 0x12345678

    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Direct(Reg::CR0), &vec![0x0F, 0x20, 0x00]); // MOV EAX, CR0
    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::CR0), Operand::Direct(Reg::EAX), &vec![0x0F, 0x22, 0x00]); // MOV CR0, EAX

    encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Direct(Reg::DR0), &vec![0x0F, 0x21, 0x00]); // MOV EAX, DR0
    // encode32_helper2(Mnemonic::MOV, Operand::Direct(Reg::DR0), Operand::Direct(Reg::EAX), &vec![0x0F, 0x23, 0x00]); // MOV DR0, EAX // TODO Fix
}

#[test]
fn instr_movapd() {
    encode32_helper2(Mnemonic::MOVAPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x28, 0xCA]); // MOVAPD XMM1, XMM2
    encode32_helper2(Mnemonic::MOVAPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x28, 0x08]); // MOVAPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVAPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x29, 0x08]); // MOVAPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVAPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0x28, 0xCA]); // VMOVAPD XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVAPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF9, 0x28, 0x08]); // VMOVAPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x29, 0x08]); // VMOVAPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x28, 0x08]); // VMOVAPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x28, 0x08]); // VMOVAPD YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x28, 0x08]); // VMOVAPD ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x28, 0xCA]); // VMOVAPD XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x28, 0xCA]); // VMOVAPD YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVAPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x28, 0xCA]); // VMOVAPD ZMM1, ZMM2
}

#[test]
fn instr_movaps() {
    encode32_helper2(Mnemonic::MOVAPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x28, 0xCA]); // MOVAPS XMM1, XMM2
    encode32_helper2(Mnemonic::MOVAPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x28, 0x08]); // MOVAPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVAPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x29, 0x08]); // MOVAPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVAPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF8, 0x28, 0xCA]); // VMOVAPS XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVAPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF8, 0x28, 0x08]); // VMOVAPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x29, 0x08]); // VMOVAPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7C, 0x0A, 0x28, 0x08]); // VMOVAPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7C, 0x2A, 0x28, 0x08]); // VMOVAPS YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7C, 0x4A, 0x28, 0x08]); // VMOVAPS ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7C, 0x0A, 0x28, 0xCA]); // VMOVAPS XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7C, 0x2A, 0x28, 0xCA]); // VMOVAPS YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVAPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0x7C, 0x4A, 0x28, 0xCA]); // VMOVAPS ZMM1, ZMM2
}

#[test]
fn instr_movbe() {
    encode32_helper2(Mnemonic::MOVBE, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0x38, 0xF0, 0x18]); // MOVBE BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVBE, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0F, 0x38, 0xF0, 0x18]); // MOVBE EBX, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVBE, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0x38, 0xF1, 0x18]); // MOVBE WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::MOVBE, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0x38, 0xF1, 0x18]); // MOVBE DWORD PTR [EAX], EBX
}

#[test]
fn instr_movd() {
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::MM0), Operand::Direct(Reg::EAX), &vec![0x0F, 0x6E, 0xC0]); // MOVD MM0, EAX
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::MM0), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0F, 0x6E, 0x00]); // MOVD MM0, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::MM0), &vec![0x0F, 0x7E, 0xC0]); // MOVD EAX, MM0
    encode32_helper2(Mnemonic::MOVD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::MM0), &vec![0x0F, 0x7E, 0x00]); // MOVD DWORD PTR [EAX], MM0
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), &vec![0x66, 0x0F, 0x6E, 0xC8]); // MOVD XMM1, EAX
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x0F, 0x6E, 0x08]); // MOVD XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x7E, 0xC8]); // MOVD EAX, XMM1
    encode32_helper2(Mnemonic::MOVD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x7E, 0x08]); // MOVD DWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), &vec![0xC5, 0xF9, 0x6E, 0xC8]); // VMOVD XMM1, EAX
    encode32_helper2(Mnemonic::VMOVD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xF9, 0x6E, 0x08]); // VMOVD XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x7E, 0xC8]); // VMOVD EAX, XMM1
    encode32_helper2(Mnemonic::VMOVD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x7E, 0x08]); // VMOVD DWORD PTR [EAX], XMM1
    // TODO Test EVEX version
}

#[test]
fn instr_movdqu() {
    encode32_helper2(Mnemonic::MOVDQU, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x6F, 0xCA]); // MOVDQU XMM1, XMM2
    encode32_helper2(Mnemonic::MOVDQU, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF3, 0x0F, 0x6F, 0x08]); // MOVDQU XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVDQU, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xF3, 0x0F, 0x7F, 0x08]); // MOVDQU XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0x6F, 0xCA]); // VMOVDQU XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFA, 0x6F, 0x08]); // VMOVDQU XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFA, 0x7F, 0x08]); // VMOVDQU XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFE, 0x6F, 0xCA]); // VMOVDQU YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFE, 0x6F, 0x08]); // VMOVDQU YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFE, 0x7F, 0x08]); // VMOVDQU YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVDQU8, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7F, 0x08, 0x6F, 0xCA]); // VMOVDQU8 XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVDQU8, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7F, 0x28, 0x6F, 0x08]); // VMOVDQU8 YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU8, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0x7F, 0x48, 0x7F, 0x08]); // VMOVDQU8 ZMMWORD PTR [EAX], ZMM1
    encode32_helper2(Mnemonic::VMOVDQU16, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFF, 0x08, 0x6F, 0xCA]); // VMOVDQU16 XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVDQU16, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFF, 0x28, 0x6F, 0x08]); // VMOVDQU16 YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU16, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0xFF, 0x48, 0x7F, 0x08]); // VMOVDQU16 ZMMWORD PTR [EAX], ZMM1
    encode32_helper2(Mnemonic::VMOVDQU32, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7E, 0x08, 0x6F, 0xCA]); // VMOVDQU32 XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVDQU32, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7E, 0x28, 0x6F, 0x08]); // VMOVDQU32 YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU32, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0x7E, 0x48, 0x7F, 0x08]); // VMOVDQU32 ZMMWORD PTR [EAX], ZMM1
    encode32_helper2(Mnemonic::VMOVDQU64, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0xFE, 0x08, 0x6F, 0xCA]); // VMOVDQU64 XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVDQU64, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFE, 0x28, 0x6F, 0x08]); // VMOVDQU64 YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVDQU64, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0xFE, 0x48, 0x7F, 0x08]); // VMOVDQU64 ZMMWORD PTR [EAX], ZMM1
}

#[test]
fn instr_movdq2q() {
    encode32_helper2(Mnemonic::MOVDQ2Q, Operand::Direct(Reg::MM3), Operand::Direct(Reg::XMM4), &vec![0xF2, 0x0F, 0xD6, 0xDC]); // MOVDQ2Q MM3, XMM4
}

#[test]
fn instr_movhlps() {
    encode32_helper2(Mnemonic::MOVHLPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x12, 0xCA]); // MOVHLPS XMM1, XMM2
    encode32_helper3(Mnemonic::VMOVHLPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x12, 0xCB]); // VMOVHLPS XMM1, XMM2, XMM3
}

#[test]
fn instr_movhpd() {
    encode32_helper2(Mnemonic::MOVHPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x16, 0x08]); // MOVHPD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVHPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xE9, 0x16, 0x08]); // VMOVHPD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVHPD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x17, 0x08]); // MOVHPD QWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVHPD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x17, 0x08]); // VMOVHPD QWORD PTR [EAX], XMM1
}

#[test]
fn instr_movhps() {
    encode32_helper2(Mnemonic::MOVHPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x16, 0x08]); // MOVHPS XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVHPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xE8, 0x16, 0x08]); // VMOVHPS XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVHPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x17, 0x08]); // MOVHPS QWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVHPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x17, 0x08]); // VMOVHPS QWORD PTR [EAX], XMM1
}

#[test]
fn instr_movlhps() {
    encode32_helper2(Mnemonic::MOVLHPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x16, 0xCA]); // MOVLHPS XMM1, XMM2
    encode32_helper3(Mnemonic::VMOVLHPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x16, 0xCB]); // VMOVLHPS XMM1, XMM2, XMM3
}

#[test]
fn instr_movlpd() {
    encode32_helper2(Mnemonic::MOVLPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x12, 0x08]); // MOVLPD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVLPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xE9, 0x12, 0x08]); // VMOVLPD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVLPD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x13, 0x08]); // MOVLPD QWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVLPD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x13, 0x08]); // VMOVLPD QWORD PTR [EAX], XMM1
}

#[test]
fn instr_movlps() {
    encode32_helper2(Mnemonic::MOVLPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x12, 0x08]); // MOVLPS XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVLPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xE8, 0x12, 0x08]); // VMOVLPS XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVLPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x13, 0x08]); // MOVLPS QWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVLPS, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x13, 0x08]); // VMOVLPS QWORD PTR [EAX], XMM1
}

#[test]
fn instr_movmskpd() {
    encode32_helper2(Mnemonic::MOVMSKPD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x50, 0xC1]); // MOVMSKPD EAX, XMM1
    encode32_helper2(Mnemonic::VMOVMSKPD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x50, 0xC1]); // VMOVMSKPD EAX, XMM1
    encode32_helper2(Mnemonic::VMOVMSKPD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFD, 0x50, 0xC1]); // VMOVMSKPD EAX, YMM1
}

#[test]
fn instr_movmskps() {
    encode32_helper2(Mnemonic::MOVMSKPS, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x50, 0xC1]); // MOVMSKPS EAX, XMM1
    encode32_helper2(Mnemonic::VMOVMSKPS, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x50, 0xC1]); // VMOVMSKPS EAX, XMM1
    encode32_helper2(Mnemonic::VMOVMSKPS, Operand::Direct(Reg::EAX), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFC, 0x50, 0xC1]); // VMOVMSKPS EAX, YMM1
}

#[test]
fn instr_movntdqa() {
    encode32_helper2(Mnemonic::MOVNTDQA, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x2A, 0x08]); // MOVNTDQA XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVNTDQA, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x79, 0x2A, 0x08]); // VMOVNTDQA XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVNTDQA, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x2A, 0x08]); // VMOVNTDQA YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVNTDQA, Operand::Direct(Reg::ZMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x7D, 0x48, 0x2A, 0x08]); // VMOVNTDQA ZMM1, ZMMWORD PTR [EAX]
}

#[test]
fn instr_movntdq() {
    encode32_helper2(Mnemonic::MOVNTDQ, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0xE7, 0x08]); // MOVNTDQ XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTDQ, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0xE7, 0x08]); // VMOVNTDQ XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTDQ, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFD, 0xE7, 0x08]); // VMOVNTDQ YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVNTDQ, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0x7D, 0x48, 0xE7, 0x08]); // VMOVNTDQ ZMMWORD PTR [EAX], ZMM1
}

#[test]
fn instr_movnti() {
    encode32_helper2(Mnemonic::MOVNTI, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EAX), &vec![0x0F, 0xC3, 0x00]); // MOVNTI QWORD PTR [EAX], EAX
}

#[test]
fn instr_movntpd() {
    encode32_helper2(Mnemonic::MOVNTPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x2B, 0x08]); // MOVNTPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x2B, 0x08]); // VMOVNTPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTPD, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFD, 0x2B, 0x08]); // VMOVNTPD YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVNTPD, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0xFD, 0x48, 0x2B, 0x08]); // VMOVNTPD ZMMWORD PTR [EAX], ZMM1
}

#[test]
fn instr_movntps() {
    encode32_helper2(Mnemonic::MOVNTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x2B, 0x08]); // MOVNTPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x2B, 0x08]); // VMOVNTPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVNTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFC, 0x2B, 0x08]); // VMOVNTPS YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVNTPS, Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0x7C, 0x48, 0x2B, 0x08]); // VMOVNTPS ZMMWORD PTR [EAX], ZMM1
}

#[test]
fn instr_movntq() {
    encode32_helper2(Mnemonic::MOVNTQ, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::MM1), &vec![0x0F, 0xE7, 0x08]); // MOVNTQ QWORD PTR [EAX], MM1
}

#[test]
fn instr_movq() {
    encode32_helper2(Mnemonic::MOVQ, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x6F, 0xCA]); // MOVQ MM1, MM2
    encode32_helper2(Mnemonic::MOVQ, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x6F, 0x08]); // MOVQ MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVQ, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::MM1), &vec![0x0F, 0x7F, 0x08]); // MOVQ QWORD PTR [EAX], MM1
    encode32_helper2(Mnemonic::MOVQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF3, 0x0F, 0x7E, 0x08]); // MOVQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0x7E, 0xCA]); // VMOVQ XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xFA, 0x7E, 0x08]); // VMOVQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVQ, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0xD6, 0x08]); // MOVQ QWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVQ, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0xD6, 0x08]); // VMOVQ QWORD PTR [EAX], XMM1
}

#[test]
fn instr_movq2dq() {
    encode32_helper2(Mnemonic::MOVQ2DQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::MM2), &vec![0xF3, 0x0F, 0xD6, 0xCA]); // MOVQ2DQ XMM1, MM2
}

#[test]
fn instr_movs() {
    encode32_helper2(Mnemonic::MOVS, Operand::Indirect(Reg::EDI, Some(OperandSize::Byte), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Byte), Some(SegmentReg::DS)), &vec![0xA4]); // MOVS BYTE PTR [S:[EDI], BYTE PTR [S:[ESI]
    encode32_helper2(Mnemonic::MOVS, Operand::Indirect(Reg::EDI, Some(OperandSize::Word), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Word), Some(SegmentReg::DS)), &vec![0x66, 0xA5]); // MOVS WORD PTR [S:[EDI], WORD PTR [S:[ESI]
    encode32_helper2(Mnemonic::MOVS, Operand::Indirect(Reg::EDI, Some(OperandSize::Dword), Some(SegmentReg::ES)), Operand::Indirect(Reg::ESI, Some(OperandSize::Dword), Some(SegmentReg::DS)), &vec![0xA5]); // MOVS DWORD PTR [S:[EDI], DWORD PTR [S:[ESI]
    encode32_helper0(Mnemonic::MOVSB, &vec![0xA4]); // MOVS BYTE PTR [S:[EDI], BYTE PTR [S:[ESI]
    encode32_helper0(Mnemonic::MOVSW, &vec![0x66, 0xA5]); // MOVS WORD PTR [S:[EDI], WORD PTR [S:[ESI]
    encode32_helper0(Mnemonic::MOVSD, &vec![0xA5]); // MOVS DWORD PTR [S:[EDI], DWORD PTR [S:[ESI]
}

#[test]
fn instr_movsd() {
    encode32_helper2(Mnemonic::MOVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x10, 0xCA]); // MOVSD XMM1, XMM2
    encode32_helper2(Mnemonic::MOVSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x10, 0x08]); // MOVSD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVSD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xF2, 0x0F, 0x11, 0x08]); // MOVSD QWORD PTR [EAX], XMM1
    encode32_helper3(Mnemonic::VMOVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x10, 0xCB]); // VMOVSD XMM1, XMM2, XMM3
    encode32_helper2(Mnemonic::VMOVSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xFB, 0x10, 0x08]); // VMOVSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x10, 0xCB]); // VMOVSD XMM1, XMM2, XMM3
    encode32_helper2(Mnemonic::VMOVSD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFB, 0x11, 0x08]); // VMOVSD QWORD PTR [EAX], XMM1
    encode32_helper3(Mnemonic::VMOVSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xEF, 0x0A, 0x10, 0xCB]); // VMOVSD XMM1, XMM2, XMM3
    encode32_helper2(Mnemonic::VMOVSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xFF, 0x0A, 0x10, 0x08]); // VMOVSD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSD, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::Qword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM1), &vec![0x62, 0xF1, 0xFF, 0x0A, 0x11, 0x08]); // VMOVSD QWORD PTR [EAX]{K2}, XMM1
}

#[test]
fn instr_movshdup() {
    encode32_helper2(Mnemonic::MOVSHDUP, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x16, 0xCA]); // MOVSHDUP XMM1, XMM2
    encode32_helper2(Mnemonic::MOVSHDUP, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF3, 0x0F, 0x16, 0x08]); // MOVSHDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0x16, 0xCA]); // VMOVSHDUP XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFA, 0x16, 0x08]); // VMOVSHDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFE, 0x16, 0xCA]); // VMOVSHDUP YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFE, 0x16, 0x08]); // VMOVSHDUP YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x16, 0xCA]); // VMOVSHDUP XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x16, 0x08]); // VMOVSHDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7E, 0x2A, 0x16, 0xCA]); // VMOVSHDUP YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7E, 0x2A, 0x16, 0x08]); // VMOVSHDUP YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0x7E, 0x4A, 0x16, 0xCA]); // VMOVSHDUP ZMM1, ZMM2
    encode32_helper2(Mnemonic::VMOVSHDUP, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7E, 0x4A, 0x16, 0x08]); // VMOVSHDUP ZMM1, ZMMWORD PTR [EAX]
}

#[test]
fn instr_movsldup() {
    encode32_helper2(Mnemonic::MOVSLDUP, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x12, 0xCA]); // MOVSLDUP XMM1, XMM2
    encode32_helper2(Mnemonic::MOVSLDUP, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xF3, 0x0F, 0x12, 0x08]); // MOVSLDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xFA, 0x12, 0xCA]); // VMOVSLDUP XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xFA, 0x12, 0x08]); // VMOVSLDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFE, 0x12, 0xCA]); // VMOVSLDUP YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFE, 0x12, 0x08]); // VMOVSLDUP YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x12, 0xCA]); // VMOVSLDUP XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x12, 0x08]); // VMOVSLDUP XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7E, 0x2A, 0x12, 0xCA]); // VMOVSLDUP YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7E, 0x2A, 0x12, 0x08]); // VMOVSLDUP YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0x7E, 0x4A, 0x12, 0xCA]); // VMOVSLDUP ZMM1, ZMM2
    encode32_helper2(Mnemonic::VMOVSLDUP, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7E, 0x4A, 0x12, 0x08]); // VMOVSLDUP ZMM1, ZMMWORD PTR [EAX]
}

#[test]
fn instr_movss() {
    encode32_helper2(Mnemonic::MOVSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x10, 0xCA]); // MOVSS XMM1, XMM2
    encode32_helper2(Mnemonic::MOVSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x10, 0x08]); // MOVSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMOVSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x10, 0xCB]); // VMOVSS XMM1, XMM2, XMM3
    encode32_helper2(Mnemonic::VMOVSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xFA, 0x10, 0x08]); // VMOVSS XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xFA, 0x11, 0x08]); // VMOVSS DWORD PTR [EAX], XMM1
    encode32_helper3(Mnemonic::VMOVSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6E, 0x0A, 0x10, 0xCB]); // VMOVSS XMM1, XMM2, XMM3
    encode32_helper2(Mnemonic::VMOVSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x10, 0x08]); // VMOVSS XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVSS, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::Dword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM1), &vec![0x62, 0xF1, 0x7E, 0x0A, 0x11, 0x08]); // VMOVSD QWORD PTR [EAX]{K2}, XMM1
}

#[test]
fn instr_movsx() {
    encode32_helper2(Mnemonic::MOVSX, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x66, 0x0F, 0xBE, 0x00]); // MOVSX AX, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::MOVSX, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x0F, 0xBE, 0x00]); // MOVSX EAX, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::MOVSX, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x0F, 0xBF, 0x00]); // MOVSX EAX, WORD PTR [EAX]
}

#[test]
fn instr_movupd() {
    encode32_helper2(Mnemonic::MOVUPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x10, 0xCA]); // MOVUPD XMM1, XMM2
    encode32_helper2(Mnemonic::MOVUPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x10, 0x08]); // MOVUPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVUPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0x11, 0x08]); // MOVUPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF9, 0x10, 0xCA]); // VMOVUPD XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF9, 0x10, 0x08]); // VMOVUPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0x11, 0x08]); // VMOVUPD XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFD, 0x10, 0xCA]); // VMOVUPD YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFD, 0x10, 0x08]); // VMOVUPD YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPD, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFD, 0x11, 0x08]); // VMOVUPD YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x10, 0x08]); // VMOVUPD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::XMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM1), &vec![0x62, 0xF1, 0xFD, 0x0A, 0x11, 0x08]); // VMOVUPD XMMWORD PTR [EAX]{K2], XMM1
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x10, 0xCA]); // VMOVUPD YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x10, 0x08]); // VMOVUPD YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::YMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM1), &vec![0x62, 0xF1, 0xFD, 0x2A, 0x11, 0x08]); // VMOVUPD YMMWORD PTR [EAX]{K2], YMM1
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x10, 0xCA]); // VMOVUPD ZMM1, ZMM2
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x10, 0x08]); // VMOVUPD ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPD, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::ZMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0xFD, 0x4A, 0x11, 0x08]); // VMOVUPD ZMMWORD PTR [EAX]{K2], ZMM1
 }

#[test]
fn instr_movups() {
    encode32_helper2(Mnemonic::MOVUPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x10, 0xCA]); // MOVUPS XMM1, XMM2
    encode32_helper2(Mnemonic::MOVUPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x10, 0x08]); // MOVUPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::MOVUPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0x0F, 0x11, 0x08]); // MOVUPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xC5, 0xF8, 0x10, 0xCA]); // VMOVUPS XMM1, XMM2
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xF8, 0x10, 0x08]); // VMOVUPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF8, 0x11, 0x08]); // VMOVUPS XMMWORD PTR [EAX], XMM1
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), &vec![0xC5, 0xFC, 0x10, 0xCA]); // VMOVUPS YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xFC, 0x10, 0x08]); // VMOVUPS YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPS, Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFC, 0x11, 0x08]); // VMOVUPS YMMWORD PTR [EAX], YMM1
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x7C, 0x0A, 0x10, 0x08]); // VMOVUPS XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::XMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM1), &vec![0x62, 0xF1, 0x7C, 0x0A, 0x11, 0x08]); // VMOVUPS XMMWORD PTR [EAX]{K2], XMM1
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), &vec![0x62, 0xF1, 0x7C, 0x2A, 0x10, 0xCA]); // VMOVUPS YMM1, YMM2
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x7C, 0x2A, 0x10, 0x08]); // VMOVUPS YMM1, YMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::YMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM1), &vec![0x62, 0xF1, 0x7C, 0x2A, 0x11, 0x08]); // VMOVUPS YMMWORD PTR [EAX]{K2], YMM1
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), &vec![0x62, 0xF1, 0x7C, 0x4A, 0x10, 0xCA]); // VMOVUPS ZMM1, ZMM2
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x7C, 0x4A, 0x10, 0x08]); // VMOVUPS ZMM1, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VMOVUPS, Operand::AVXDestinationIndirect(Reg::EAX, Some(OperandSize::ZMMword), None, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM1), &vec![0x62, 0xF1, 0x7C, 0x4A, 0x11, 0x08]); // VMOVUPS ZMMWORD PTR [EAX]{K2], ZMM1
}

#[test]
fn instr_movzx() {
    encode32_helper2(Mnemonic::MOVZX, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x66, 0x0F, 0xB6, 0x00]); // MOVZX AX, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::MOVZX, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x0F, 0xB6, 0x00]); // MOVZX EAX, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::MOVZX, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x0F, 0xB7, 0x00]); // MOVZX EAX, WORD PTR [EAX]
}

#[test]
fn instr_mpsadbw() {
    encode32_helper3(Mnemonic::MPSADBW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x5), &vec![0x66, 0x0F, 0x3A, 0x42, 0xCA, 0x05]); // MPSADBW XMM1, XMM2, 0x5
    encode32_helper3(Mnemonic::MPSADBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x5), &vec![0x66, 0x0F, 0x3A, 0x42, 0x08, 0x05]); // MPSADBW XMM1, XMMWORD PTR [EAX], 0x5
    encode32_helper4(Mnemonic::VMPSADBW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x5), &vec![0xC4, 0xE3, 0x69, 0x42, 0xCB, 0x05]); // VMPSADBW XMM1, XMM2, XMM3, 0x5
    encode32_helper4(Mnemonic::VMPSADBW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x5), &vec![0xC4, 0xE3, 0x69, 0x42, 0x08, 0x05]); // VMPSADBW XMM1, XMM2, XMMWORD PTR [EAX], 0x5
    encode32_helper4(Mnemonic::VMPSADBW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x5), &vec![0xC4, 0xE3, 0x6D, 0x42, 0xCB, 0x05]); // VMPSADBW YMM1, YMM2, YMM3, 0x5
    encode32_helper4(Mnemonic::VMPSADBW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x5), &vec![0xC4, 0xE3, 0x6D, 0x42, 0x08, 0x05]); // VMPSADBW YMM1, YMM2, YMMWORD PTR [EAX], 0x5
}

#[test]
fn instr_mul() {
    encode32_helper1(Mnemonic::MUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xF6, 0x20]); // MUL BYTE PTR [EAX]
    encode32_helper1(Mnemonic::MUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF7, 0x20]); // MUL WORD PTR [EAX]
    encode32_helper1(Mnemonic::MUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF7, 0x20]); // MUL DWORD PTR [EAX]
}

#[test]
fn instr_mulpd() {
    encode32_helper2(Mnemonic::MULPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x59, 0xCA]); // MULPD XMM1, XMM2
    encode32_helper2(Mnemonic::MULPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x59, 0x08]); // MULPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x59, 0xCB]); // VMULPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x59, 0x08]); // VMULPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x59, 0xCB]); // VMULPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMULPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x59, 0x08]); // VMULPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xED, 0x0C, 0x59, 0xCB]); // VMULPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x0C, 0x59, 0x08]); // VMULPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0x2C, 0x59, 0xCB]); // VMULPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0x2C, 0x59, 0x08]); // VMULPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0xED, 0x4C, 0x59, 0xCB]); // VMULPD ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VMULPD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x4C, 0x59, 0x08]); // VMULPD ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_mulps() {
    encode32_helper2(Mnemonic::MULPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x59, 0xCA]); // MULPS XMM1, XMM2
    encode32_helper2(Mnemonic::MULPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x59, 0x08]); // MULPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x59, 0xCB]); // VMULPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x59, 0x08]); // VMULPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x59, 0xCB]); // VMULPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMULPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x59, 0x08]); // VMULPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6C, 0x0C, 0x59, 0xCB]); // VMULPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6C, 0x0C, 0x59, 0x08]); // VMULPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6C, 0x2C, 0x59, 0xCB]); // VMULPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6C, 0x2C, 0x59, 0x08]); // VMULPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6C, 0x4C, 0x59, 0xCB]); // VMULPS ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VMULPS, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K4), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6C, 0x4C, 0x59, 0x08]); // VMULPS ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_mulsd() {
    encode32_helper2(Mnemonic::MULSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF2, 0x0F, 0x59, 0xCA]); // MULSD XMM1, XMM2
    encode32_helper2(Mnemonic::MULSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xF2, 0x0F, 0x59, 0x08]); // MULSD XMM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEB, 0x59, 0xCB]); // VMULSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC5, 0xEB, 0x59, 0x08]); // VMULSD XMM1, XMM2, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0xEF, 0x0A, 0x59, 0xCB]); // VMULSD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xEF, 0x0A, 0x59, 0x08]); // VMULSD XMM1, XMM2, QWORD PTR [EAX]
}

#[test]
fn instr_mulss() {
    encode32_helper2(Mnemonic::MULSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0xF3, 0x0F, 0x59, 0xCA]); // MULSS XMM1, XMM2
    encode32_helper2(Mnemonic::MULSS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0x59, 0x08]); // MULSS XMM1, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xEA, 0x59, 0xCB]); // VMULSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULSS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC5, 0xEA, 0x59, 0x08]); // VMULSS XMM1, XMM2, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::VMULSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6E, 0x0A, 0x59, 0xCB]); // VMULSS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VMULSS, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6E, 0x0A, 0x59, 0x08]); // VMULSS XMM1, XMM2, DWORD PTR [EAX]
}

#[test]
fn instr_mulx() {
    encode32_helper3(Mnemonic::MULX, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x63, 0xF6, 0x00]); // MULX EAX, EBX, DWORD PTR [EAX]
}

#[test]
fn instr_mwait() {
    encode32_helper0(Mnemonic::MWAIT,  &vec![0x0F, 0x01, 0xC9]); // MWAIT 
}

#[test]
fn instr_neg() {
    encode32_helper1(Mnemonic::NEG, Operand::Direct(Reg::AL), &vec![0xF6, 0xD8]); // NEG AL
    encode32_helper1(Mnemonic::NEG, Operand::Direct(Reg::AX), &vec![0x66, 0xF7, 0xD8]); // NEG AX
    encode32_helper1(Mnemonic::NEG, Operand::Direct(Reg::EAX), &vec![0xF7, 0xD8]); // NEG EAX
}

#[test]
fn instr_nop() {
    encode32_helper0(Mnemonic::NOP,  &vec![0x90]); // NOP 
    encode32_helper1(Mnemonic::NOP, Operand::Direct(Reg::AX), &vec![0x66, 0x0F, 0x1F, 0xC0]); // NOP AX
    encode32_helper1(Mnemonic::NOP, Operand::Direct(Reg::EAX), &vec![0x0F, 0x1F, 0xC0]); // NOP EAX
}

#[test]
fn instr_or() {
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x0C, 0x12]); // OR AL, 0x12
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x0D, 0x34, 0x12]); // OR AX, 0x1234
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x0D, 0x78, 0x56, 0x34, 0x12]); // OR EAX, 0x12345678
    encode32_helper2(Mnemonic::OR, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x08, 0x12]); // OR BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::OR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x08, 0x34, 0x12]); // OR WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::OR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x08, 0x78, 0x56, 0x34, 0x12]); // OR DWORD PTR [EAX], 0x12345678
    encode32_helper2(Mnemonic::OR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x08, 0x12]); // OR WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::OR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x83, 0x08, 0x12]); // OR DWORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x09, 0xCB]); // OR BX, CX
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x09, 0xCB]); // OR EBX, ECX
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x0A, 0x18]); // OR BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0B, 0x18]); // OR BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::OR, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0B, 0x18]); // OR EBX, DWORD PTR [EAX]
}

#[test]
fn instr_orpd() {
    encode32_helper2(Mnemonic::ORPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x56, 0xCA]); // ORPD XMM1, XMM2
    encode32_helper2(Mnemonic::ORPD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x56, 0x08]); // ORPD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x56, 0xCB]); // VORPD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x56, 0x08]); // VORPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x56, 0xCB]); // VORPD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x56, 0x08]); // VORPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x18, 0x56, 0x08]); // VORPD XMM1, XMM2, QWORD PTR [EAX]{1to2}
encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x38, 0x56, 0x08]); // VORPD YMM1, YMM2, QWORD PTR [EAX]{1to4}
encode32_helper3(Mnemonic::VORPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x58, 0x56, 0x08]); // VORPD ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_orps() {
    encode32_helper2(Mnemonic::ORPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x0F, 0x56, 0xCA]); // ORPS XMM1, XMM2
    encode32_helper2(Mnemonic::ORPS, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x0F, 0x56, 0x08]); // ORPS XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE8, 0x56, 0xCB]); // VORPS XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE8, 0x56, 0x08]); // VORPS XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xEC, 0x56, 0xCB]); // VORPS YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xEC, 0x56, 0x08]); // VORPS YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x18, 0x56, 0x08]); // VORPS XMM1, XMM2, DWORD PTR [EAX]{1to4}
encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x38, 0x56, 0x08]); // VORPS YMM1, YMM2, DWORD PTR [EAX]{1to8}
encode32_helper3(Mnemonic::VORPS, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6C, 0x58, 0x56, 0x08]); // VORPS ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_out() {
 encode32_helper2(Mnemonic::OUT, Operand::Literal8(0x12), Operand::Direct(Reg::AL), &vec![0xE6, 0x12]); // OUT 0x12, AL
 encode32_helper2(Mnemonic::OUT, Operand::Literal8(0x12), Operand::Direct(Reg::AX), &vec![0x66, 0xE7, 0x12]); // OUT 0x12, AX
 encode32_helper2(Mnemonic::OUT, Operand::Literal8(0x12), Operand::Direct(Reg::EAX), &vec![0xE7, 0x12]); // OUT 0x12, EAX
 encode32_helper2(Mnemonic::OUT, Operand::Direct(Reg::DX), Operand::Direct(Reg::AL), &vec![0xEE]); // OUT DX, AL
 encode32_helper2(Mnemonic::OUT, Operand::Direct(Reg::DX), Operand::Direct(Reg::AX), &vec![0x66, 0xEF]); // OUT DX, AX
 encode32_helper2(Mnemonic::OUT, Operand::Direct(Reg::DX), Operand::Direct(Reg::EAX), &vec![0xEF]); // OUT DX, EAX
}

#[test]
fn instr_outs() {
    encode32_helper2(Mnemonic::OUTS, Operand::Direct(Reg::DX), Operand::Indirect(Reg::ESI, Some(OperandSize::Byte), Some(SegmentReg::DS)), &vec![0x6E]); // OUTS DX, BYTE PTR [S:[ESI]
    encode32_helper2(Mnemonic::OUTS, Operand::Direct(Reg::DX), Operand::Indirect(Reg::ESI, Some(OperandSize::Word), Some(SegmentReg::DS)), &vec![0x66, 0x6F]); // OUTS DX, WORD PTR [S:[ESI]
    encode32_helper2(Mnemonic::OUTS, Operand::Direct(Reg::DX), Operand::Indirect(Reg::ESI, Some(OperandSize::Dword), Some(SegmentReg::DS)), &vec![0x6F]); // OUTS DX, DWORD PTR [S:[ESI]
    encode32_helper0(Mnemonic::OUTSB, &vec![0x6E]); // OUTSB
    encode32_helper0(Mnemonic::OUTSW, &vec![0x66, 0x6F]); // OUTSW
    encode32_helper0(Mnemonic::OUTSD, &vec![0x6F]); // OUTSD
}

#[test]
fn instr_packss() {
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x63, 0xCB]); // VPACKSSWB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x63, 0x08]); // VPACKSSWB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x6B, 0xCB]); // VPACKSSDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x6B, 0x08]); // VPACKSSDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x63, 0xCB]); // VPACKSSWB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x63, 0x08]); // VPACKSSWB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x6B, 0xCB]); // VPACKSSDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x6B, 0x08]); // VPACKSSDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x63, 0xCB]); // VPACKSSWB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x63, 0x08]); // VPACKSSWB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x63, 0xCB]); // VPACKSSWB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x63, 0x08]); // VPACKSSWB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x63, 0xCB]); // VPACKSSWB ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPACKSSWB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x63, 0x08]); // VPACKSSWB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x6B, 0xCB]); // VPACKSSDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x6B, 0x08]); // VPACKSSDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x1B, 0x6B, 0x08]); // VPACKSSDW XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x6B, 0xCB]); // VPACKSSDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x6B, 0x08]); // VPACKSSDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x3B, 0x6B, 0x08]); // VPACKSSDW YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x6B, 0xCB]); // VPACKSSDW ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x6B, 0x08]); // VPACKSSDW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKSSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x5B, 0x6B, 0x08]); // VPACKSSDW ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_packusdw() {
    encode32_helper2(Mnemonic::PACKUSDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x2B, 0xCA]); // PACKUSDW XMM1, XMM2
    encode32_helper2(Mnemonic::PACKUSDW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x2B, 0x08]); // PACKUSDW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x2B, 0xCB]); // VPACKUSDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x2B, 0x08]); // VPACKUSDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x2B, 0xCB]); // VPACKUSDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x2B, 0x08]); // VPACKUSDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF2, 0x6D, 0x0B, 0x2B, 0xCB]); // VPACKUSDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0B, 0x2B, 0x08]); // VPACKUSDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x1B, 0x2B, 0x08]); // VPACKUSDW XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF2, 0x6D, 0x2B, 0x2B, 0xCB]); // VPACKUSDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2B, 0x2B, 0x08]); // VPACKUSDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x3B, 0x2B, 0x08]); // VPACKUSDW YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF2, 0x6D, 0x4B, 0x2B, 0xCB]); // VPACKUSDW ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x4B, 0x2B, 0x08]); // VPACKUSDW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x5B, 0x2B, 0x08]); // VPACKUSDW ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
}

#[test]
fn instr_packuswb() {
    encode32_helper2(Mnemonic::PACKUSWB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x67, 0xCA]); // PACKUSWB MM1, MM2
    encode32_helper2(Mnemonic::PACKUSWB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x67, 0x08]); // PACKUSWB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PACKUSWB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x67, 0xCA]); // PACKUSWB XMM1, XMM2
    encode32_helper2(Mnemonic::PACKUSWB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x67, 0x08]); // PACKUSWB XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x67, 0xCB]); // VPACKUSWB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x67, 0x08]); // VPACKUSWB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x67, 0xCB]); // VPACKUSWB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x67, 0x08]); // VPACKUSWB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x67, 0xCB]); // VPACKUSWB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0x67, 0x08]); // VPACKUSWB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x67, 0xCB]); // VPACKUSWB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0x67, 0x08]); // VPACKUSWB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x67, 0xCB]); // VPACKUSWB ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPACKUSWB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0x67, 0x08]); // VPACKUSWB ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_padd() {
    encode32_helper2(Mnemonic::PADDB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xFC, 0xCA]); // PADDB MM1, MM2
    encode32_helper2(Mnemonic::PADDB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xFC, 0x08]); // PADDB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xFD, 0xCA]); // PADDW MM1, MM2
    encode32_helper2(Mnemonic::PADDW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xFD, 0x08]); // PADDW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDD, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xFE, 0xCA]); // PADDD MM1, MM2
    encode32_helper2(Mnemonic::PADDD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xFE, 0x08]); // PADDD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDQ, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xD4, 0xCA]); // PADDQ MM1, MM2
    encode32_helper2(Mnemonic::PADDQ, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xD4, 0x08]); // PADDQ MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xFC, 0x08]); // PADDB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xFD, 0x08]); // PADDW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xFE, 0x08]); // PADDD XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xD4, 0x08]); // PADDQ XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xFC, 0x08]); // VPADDB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xFD, 0x08]); // VPADDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xFE, 0x08]); // VPADDD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xD4, 0x08]); // VPADDQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xFC, 0x08]); // VPADDB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xFD, 0x08]); // VPADDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xFE, 0x08]); // VPADDD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xD4, 0x08]); // VPADDQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xFC, 0xCB]); // VPADDB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xFD, 0x08]); // VPADDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xFE, 0xCB]); // VPADDD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x0A, 0xD4, 0x08]); // VPADDQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xFC, 0xCB]); // VPADDB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xFD, 0x08]); // VPADDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xFE, 0xCB]); // VPADDD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0x2A, 0xD4, 0x08]); // VPADDQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xFC, 0xCB]); // VPADDB ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xFD, 0x08]); // VPADDW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xFE, 0xCB]); // VPADDD ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x4A, 0xD4, 0x08]); // VPADDQ ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_padds() {
    encode32_helper2(Mnemonic::PADDSB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xEC, 0xCA]); // PADDSB MM1, MM2
    encode32_helper2(Mnemonic::PADDSB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xEC, 0x08]); // PADDSB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xEC, 0xCA]); // PADDSB XMM1, XMM2
    encode32_helper2(Mnemonic::PADDSB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xEC, 0x08]); // PADDSB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xED, 0xCA]); // PADDSW MM1, MM2
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xED, 0x08]); // PADDSW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xED, 0xCA]); // PADDSW XMM1, XMM2
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xED, 0x08]); // PADDSW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xEC, 0xCB]); // VPADDSB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xEC, 0x08]); // VPADDSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xED, 0xCB]); // VPADDSW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xED, 0x08]); // VPADDSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xEC, 0xCB]); // VPADDSB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xEC, 0x08]); // VPADDSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xED, 0xCB]); // VPADDSW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xED, 0x08]); // VPADDSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xEC, 0xCB]); // VPADDSB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xEC, 0x08]); // VPADDSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xED, 0xCB]); // VPADDSW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xED, 0x08]); // VPADDSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xEC, 0xCB]); // VPADDSB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xEC, 0x08]); // VPADDSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xED, 0xCB]); // VPADDSW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xED, 0x08]); // VPADDSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xEC, 0xCB]); // VPADDSB ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDSB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xEC, 0x08]); // VPADDSB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xED, 0xCB]); // VPADDSW ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xED, 0x08]); // VPADDSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_paddusb() {
    encode32_helper2(Mnemonic::PADDUSB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xDC, 0xCA]); // PADDUSB MM1, MM2
    encode32_helper2(Mnemonic::PADDUSB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xDC, 0x08]); // PADDUSB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDUSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xDC, 0xCA]); // PADDUSB XMM1, XMM2
    encode32_helper2(Mnemonic::PADDUSB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xDC, 0x08]); // PADDUSB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xED, 0xCA]); // PADDSW MM1, MM2
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xED, 0x08]); // PADDSW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xED, 0xCA]); // PADDSW XMM1, XMM2
    encode32_helper2(Mnemonic::PADDSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xED, 0x08]); // PADDSW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDUSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xDC, 0xCB]); // VPADDUSB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDUSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDC, 0x08]); // VPADDUSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xED, 0xCB]); // VPADDSW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xED, 0x08]); // VPADDSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDUSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xDC, 0xCB]); // VPADDUSB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDUSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xDC, 0x08]); // VPADDUSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xED, 0xCB]); // VPADDSW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xED, 0x08]); // VPADDSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xDC, 0xCB]); // VPADDUSB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xDC, 0x08]); // VPADDUSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xED, 0xCB]); // VPADDSW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0B, 0xED, 0x08]); // VPADDSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xDC, 0xCB]); // VPADDUSB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xDC, 0x08]); // VPADDUSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xED, 0xCB]); // VPADDSW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2B, 0xED, 0x08]); // VPADDSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xDC, 0xCB]); // VPADDUSB ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDUSB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xDC, 0x08]); // VPADDUSB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xED, 0xCB]); // VPADDSW ZMM1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPADDSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4B, 0xED, 0x08]); // VPADDSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn palignr() {
    encode32_helper3(Mnemonic::PALIGNR, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), Operand::Literal8(0x12), &vec![0x0F, 0x3A, 0x0F, 0xCA, 0x12]); // PALIGNR MM1, MM2, 0x12
    encode32_helper3(Mnemonic::PALIGNR, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), Operand::Literal8(0x12), &vec![0x0F, 0x3A, 0x0F, 0x08, 0x12]); // PALIGNR MM1, QWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::PALIGNR, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0F, 0xCA, 0x12]); // PALIGNR XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::PALIGNR, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x0F, 0x08, 0x12]); // PALIGNR XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0F, 0xCB, 0x12]); // VPALIGNR XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x0F, 0x08, 0x12]); // VPALIGNR XMM1, XMM2, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0F, 0xCB, 0x12]); // VPALIGNR YMM1, YMM2, YMM3, 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x6D, 0x0F, 0x08, 0x12]); // VPALIGNR YMM1, YMM2, YMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x0D, 0x0F, 0xCB, 0x12]); // VPALIGNR XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x0D, 0x0F, 0x08, 0x12]); // VPALIGNR XMM1, XMM2, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x2D, 0x0F, 0xCB, 0x12]); // VPALIGNR YMM1, YMM2, YMM3, 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x2D, 0x0F, 0x08, 0x12]); // VPALIGNR YMM1, YMM2, YMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x4D, 0x0F, 0xCB, 0x12]); // VPALIGNR ZMM1, ZMM2, ZMM3, 0x12
    encode32_helper4(Mnemonic::VPALIGNR, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K5), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), Operand::Literal8(0x12), &vec![0x62, 0xF3, 0x6D, 0x4D, 0x0F, 0x08, 0x12]); // VPALIGNR ZMM1, ZMM2, ZMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_pand() {
    encode32_helper2(Mnemonic::PAND, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xDB, 0xCA]); // PAND MM1, MM2
    encode32_helper2(Mnemonic::PAND, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xDB, 0x08]); // PAND MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PAND, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xDB, 0xCA]); // PAND XMM1, XMM2
    encode32_helper2(Mnemonic::PAND, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xDB, 0x08]); // PAND XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAND, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDB, 0x08]); // VPAND XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAND, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xDB, 0xCB]); // VPAND YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPANDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x18, 0xDB, 0x08]); // VPANDD XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPANDD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xDB, 0xCB]); // VPANDD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPANDD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x48, 0xDB, 0x08]); // VPANDD ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPANDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x18, 0xDB, 0x08]); // VPANDQ XMM1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VPANDQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0x2A, 0xDB, 0xCB]); // VPANDQ YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPANDQ, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x48, 0xDB, 0x08]); // VPANDQ ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pandn() {
    encode32_helper2(Mnemonic::PANDN, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xDF, 0xCA]); // PANDN MM1, MM2
    encode32_helper2(Mnemonic::PANDN, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xDF, 0x08]); // PANDN MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PANDN, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xDF, 0xCA]); // PANDN XMM1, XMM2
    encode32_helper2(Mnemonic::PANDN, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xDF, 0x08]); // PANDN XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPANDN, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDF, 0x08]); // VPANDN XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPANDN, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xDF, 0xCB]); // VPANDN YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPANDND, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x18, 0xDF, 0x08]); // VPANDND XMM1, XMM2, DWORD PTR [EAX]{1to4}
encode32_helper3(Mnemonic::VPANDND, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xDF, 0xCB]); // VPANDND YMM1, YMM2, YMM3
encode32_helper3(Mnemonic::VPANDND, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x48, 0xDF, 0x08]); // VPANDND ZMM1, ZMM2, ZMMWORD PTR [EAX]
encode32_helper3(Mnemonic::VPANDNQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF1, 0xED, 0x18, 0xDF, 0x08]); // VPANDNQ XMM1, XMM2, QWORD PTR [EAX]{1to2}
encode32_helper3(Mnemonic::VPANDNQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0xED, 0x2A, 0xDF, 0xCB]); // VPANDNQ YMM1, YMM2, YMM3
encode32_helper3(Mnemonic::VPANDNQ, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x48, 0xDF, 0x08]); // VPANDNQ ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pause() {
    encode32_helper0(Mnemonic::PAUSE,  &vec![0xF3, 0x90]); // PAUSE 
}

#[test]
fn instr_pavg() {
    encode32_helper2(Mnemonic::PAVGB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xE0, 0xCA]); // PAVGB MM1, MM2
    encode32_helper2(Mnemonic::PAVGB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xE0, 0x08]); // PAVGB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PAVGB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xE0, 0xCA]); // PAVGB XMM1, XMM2
    encode32_helper2(Mnemonic::PAVGB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xE0, 0x08]); // PAVGB XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xE0, 0xCB]); // VPAVGB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xE0, 0x08]); // VPAVGB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xE0, 0xCB]); // VPAVGB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE0, 0x08]); // VPAVGB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0E, 0xE0, 0xCB]); // VPAVGB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE0, 0x08]); // VPAVGB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGB, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x48, 0xE0, 0xCB]); // VPAVGB ZMM1, ZMM2, ZMM3
    encode32_helper2(Mnemonic::PAVGW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0xE3, 0xCA]); // PAVGW MM1, MM2
    encode32_helper2(Mnemonic::PAVGW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xE3, 0x08]); // PAVGW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PAVGW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0xE3, 0xCA]); // PAVGW XMM1, XMM2
    encode32_helper2(Mnemonic::PAVGW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xE3, 0x08]); // PAVGW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0xE3, 0xCB]); // VPAVGW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xE3, 0x08]); // VPAVGW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0xE3, 0xCB]); // VPAVGW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE3, 0x08]); // VPAVGW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0E, 0xE3, 0xCB]); // VPAVGW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE3, 0x08]); // VPAVGW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPAVGW, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x48, 0xE3, 0xCB]); // VPAVGW ZMM1, ZMM2, ZMM3
}

#[test]
fn instr_pblendvb() {
    encode32_helper2(Mnemonic::PBLENDVB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x10, 0xCA]); // PBLENDVB XMM1, XMM2
    encode32_helper2(Mnemonic::PBLENDVB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x10, 0x08]); // PBLENDVB XMM1, XMMWORD PTR [EAX]
    encode32_helper4(Mnemonic::VPBLENDVB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4C, 0xCB, 0x40]); // VPBLENDVB XMM1, XMM2, XMM3, XMM4
    encode32_helper4(Mnemonic::VPBLENDVB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Direct(Reg::XMM4), &vec![0xC4, 0xE3, 0x69, 0x4C, 0x08, 0x40]); // VPBLENDVB XMM1, XMM2, XMMWORD PTR [EAX], XMM4
    encode32_helper4(Mnemonic::VPBLENDVB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4C, 0xCB, 0x40]); // VPBLENDVB YMM1, YMM2, YMM3, YMM4
    encode32_helper4(Mnemonic::VPBLENDVB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), Operand::Direct(Reg::YMM4), &vec![0xC4, 0xE3, 0x6D, 0x4C, 0x08, 0x40]); // VPBLENDVB YMM1, YMM2, YMMWORD PTR [EAX], YMM4
}

#[test]
fn instr_pclmulqdq() {
    encode32_helper3(Mnemonic::PCLMULQDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x44, 0xCA, 0x12]); // PCLMULQDQ XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::PCLMULQDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x44, 0x08, 0x12]); // PCLMULQDQ XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper4(Mnemonic::VPCLMULQDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x44, 0xCB, 0x12]); // VPCLMULQDQ XMM1, XMM2, XMM3, 0x12
    encode32_helper4(Mnemonic::VPCLMULQDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x69, 0x44, 0x08, 0x12]); // VPCLMULQDQ XMM1, XMM2, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_pcmpeq() {
    encode32_helper2(Mnemonic::PCMPEQB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x74, 0xCA]); // PCMPEQB MM1, MM2
    encode32_helper2(Mnemonic::PCMPEQB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x74, 0x08]); // PCMPEQB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PCMPEQB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x74, 0xCA]); // PCMPEQB XMM1, XMM2
    encode32_helper2(Mnemonic::PCMPEQB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x74, 0x08]); // PCMPEQB XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x74, 0xCB]); // VPCMPEQB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x74, 0x08]); // VPCMPEQB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x74, 0xCB]); // VPCMPEQB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x74, 0x08]); // VPCMPEQB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x74, 0xCB]); // VPCMPEQB K1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x74, 0x08]); // VPCMPEQB K1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x74, 0xCB]); // VPCMPEQB K1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x74, 0x08]); // VPCMPEQB K1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x74, 0xCB]); // VPCMPEQB K1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPCMPEQB, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x74, 0x08]); // VPCMPEQB K1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PCMPEQW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x75, 0xCA]); // PCMPEQW MM1, MM2
    encode32_helper2(Mnemonic::PCMPEQW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x75, 0x08]); // PCMPEQW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PCMPEQW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x75, 0xCA]); // PCMPEQW XMM1, XMM2
    encode32_helper2(Mnemonic::PCMPEQW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x75, 0x08]); // PCMPEQW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x75, 0xCB]); // VPCMPEQW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x75, 0x08]); // VPCMPEQW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x75, 0xCB]); // VPCMPEQW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x75, 0x08]); // VPCMPEQW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x75, 0xCB]); // VPCMPEQW K1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x75, 0x08]); // VPCMPEQW K1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x75, 0xCB]); // VPCMPEQW K1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x75, 0x08]); // VPCMPEQW K1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x75, 0xCB]); // VPCMPEQW K1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPCMPEQW, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x75, 0x08]); // VPCMPEQW K1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PCMPEQD, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x76, 0xCA]); // PCMPEQD MM1, MM2
    encode32_helper2(Mnemonic::PCMPEQD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x76, 0x08]); // PCMPEQD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PCMPEQD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x76, 0xCA]); // PCMPEQD XMM1, XMM2
    encode32_helper2(Mnemonic::PCMPEQD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x76, 0x08]); // PCMPEQD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x76, 0xCB]); // VPCMPEQD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x76, 0x08]); // VPCMPEQD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x76, 0xCB]); // VPCMPEQD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x76, 0x08]); // VPCMPEQD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x76, 0xCB]); // VPCMPEQD K1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0x76, 0x08]); // VPCMPEQD K1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x76, 0xCB]); // VPCMPEQD K1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0x76, 0x08]); // VPCMPEQD K1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x76, 0xCB]); // VPCMPEQD K1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPCMPEQD, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0x76, 0x08]); // VPCMPEQD K1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_vpcmpeqq() {
    encode32_helper2(Mnemonic::PCMPEQQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x29, 0xCA]); // PCMPEQQ XMM1, XMM2
    encode32_helper2(Mnemonic::PCMPEQQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x29, 0x08]); // PCMPEQQ XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x29, 0xCB]); // VPCMPEQQ XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x29, 0x08]); // VPCMPEQQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x29, 0xCB]); // VPCMPEQQ YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x29, 0x08]); // VPCMPEQQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF2, 0xED, 0x0A, 0x29, 0xCB]); // VPCMPEQQ K1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x0A, 0x29, 0x08]); // VPCMPEQQ K1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF2, 0xED, 0x2A, 0x29, 0xCB]); // VPCMPEQQ K1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x2A, 0x29, 0x08]); // VPCMPEQQ K1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF2, 0xED, 0x4A, 0x29, 0xCB]); // VPCMPEQQ K1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPCMPEQQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0xED, 0x4A, 0x29, 0x08]); // VPCMPEQQ K1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pcmpestri() {
    encode32_helper3(Mnemonic::PCMPESTRI, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x61, 0xCA, 0x12]); // PCMPESTRI XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::PCMPESTRI, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x61, 0x08, 0x12]); // PCMPESTRI XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::VPCMPESTRI, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x61, 0xCA, 0x12]); // VPCMPESTRI XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::VPCMPESTRI, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x61, 0x08, 0x12]); // VPCMPESTRI XMM1, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_pcmpestrm() {
    encode32_helper3(Mnemonic::PCMPESTRM, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x60, 0xCA, 0x12]); // PCMPESTRM XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::PCMPESTRM, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x60, 0x08, 0x12]); // PCMPESTRM XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::VPCMPESTRM, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x60, 0xCA, 0x12]); // VPCMPESTRM XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::VPCMPESTRM, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x60, 0x08, 0x12]); // VPCMPESTRM XMM1, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_pcmpgt() {
    encode32_helper2(Mnemonic::PCMPGTB, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x64, 0xCA]); // PCMPGTB MM1, MM2
    encode32_helper2(Mnemonic::PCMPGTB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x64, 0x08]); // PCMPGTB MM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x64, 0xCB]); // VPCMPGTB XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x64, 0x08]); // VPCMPGTB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x64, 0xCB]); // VPCMPGTB YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x64, 0x08]); // VPCMPGTB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::K2), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x08, 0x64, 0xD3]); // VPCMPGTB K2, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::AVXDestination(Reg::K3, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2C, 0x64, 0x18]); // VPCMPGTB K3, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTB, Operand::Direct(Reg::K4), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x48, 0x64, 0xE3]); // VPCMPGTB K4, ZMM2, ZMM3
    encode32_helper2(Mnemonic::PCMPGTW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x65, 0xCA]); // PCMPGTW MM1, MM2
    encode32_helper2(Mnemonic::PCMPGTW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x65, 0x08]); // PCMPGTW MM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x65, 0xCB]); // VPCMPGTW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x65, 0x08]); // VPCMPGTW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x65, 0xCB]); // VPCMPGTW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x65, 0x08]); // VPCMPGTW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::K2), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x08, 0x65, 0xD3]); // VPCMPGTW K2, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::AVXDestination(Reg::K3, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2C, 0x65, 0x18]); // VPCMPGTW K3, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTW, Operand::Direct(Reg::K4), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF1, 0x6D, 0x48, 0x65, 0xE3]); // VPCMPGTW K4, ZMM2, ZMM3
    encode32_helper2(Mnemonic::PCMPGTD, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x66, 0xCA]); // PCMPGTD MM1, MM2
    encode32_helper2(Mnemonic::PCMPGTD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x66, 0x08]); // PCMPGTD MM1, QWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC5, 0xE9, 0x66, 0xCB]); // VPCMPGTD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0x66, 0x08]); // VPCMPGTD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC5, 0xED, 0x66, 0xCB]); // VPCMPGTD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0x66, 0x08]); // VPCMPGTD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::K2), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF1, 0x6D, 0x08, 0x66, 0xD3]); // VPCMPGTD K2, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::AVXDestination(Reg::K3, Some(MaskReg::K4), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2C, 0x66, 0x18]); // VPCMPGTD K3, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTD, Operand::Direct(Reg::K4), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF1, 0x6D, 0x58, 0x66, 0x20]); // VPCMPGTD K4, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper2(Mnemonic::PCMPGTQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x37, 0xCA]); // PCMPGTQ XMM1, XMM2
    encode32_helper2(Mnemonic::PCMPGTQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x37, 0x08]); // PCMPGTQ XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x37, 0xCB]); // VPCMPGTQ XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x37, 0x08]); // VPCMPGTQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x37, 0xCB]); // VPCMPGTQ YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x37, 0x08]); // VPCMPGTQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0x62, 0xF2, 0xED, 0x08, 0x37, 0xCB]); // VPCMPGTQ K1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x08, 0x37, 0x08]); // VPCMPGTQ K1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x1A, 0x37, 0x08]); // VPCMPGTQ K1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0x62, 0xF2, 0xED, 0x28, 0x37, 0xCB]); // VPCMPGTQ K1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x28, 0x37, 0x08]); // VPCMPGTQ K1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x3A, 0x37, 0x08]); // VPCMPGTQ K1, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::ZMM2), Operand::Direct(Reg::ZMM3), &vec![0x62, 0xF2, 0xED, 0x48, 0x37, 0xCB]); // VPCMPGTQ K1, ZMM2, ZMM3
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::Direct(Reg::K1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0xED, 0x48, 0x37, 0x08]); // VPCMPGTQ K1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPCMPGTQ, Operand::AVXDestination(Reg::K1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x5A, 0x37, 0x08]); // VPCMPGTQ K1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_pcmpistri() {
    encode32_helper3(Mnemonic::PCMPISTRI, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x63, 0xCA, 0x12]); // PCMPISTRI XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::PCMPISTRI, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0x3A, 0x63, 0x08, 0x12]); // PCMPISTRI XMM1, XMMWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::VPCMPISTRI, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x63, 0xCA, 0x12]); // VPCMPISTRI XMM1, XMM2, 0x12
    encode32_helper3(Mnemonic::VPCMPISTRI, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), Operand::Literal8(0x12), &vec![0xC4, 0xE3, 0x79, 0x63, 0x08, 0x12]); // VPCMPISTRI XMM1, XMMWORD PTR [EAX], 0x12
}

#[test]
fn instr_pdep() {
    encode32_helper3(Mnemonic::PDEP, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x63, 0xF5, 0x00]); // PDEP EAX, EBX, DWORD PTR [EAX]
}

#[test]
fn instr_pext() {
    encode32_helper3(Mnemonic::PEXT, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x62, 0xF5, 0x00]); // PEXT EAX, EBX, DWORD PTR [EAX]
}

#[test]
fn instr_pextr() {
    encode32_helper3(Mnemonic::PEXTRB, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0x3A, 0x14, 0x10, 0x01]); // PEXTRB BYTE PTR [EAX], XMM2, 0x1
    encode32_helper3(Mnemonic::PEXTRW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0x3A, 0x15, 0x10, 0x01]); // PEXTRW WORD PTR [EAX], XMM2, 0x1
    encode32_helper3(Mnemonic::PEXTRD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0x3A, 0x16, 0x10, 0x01]); // PEXTRD DWORD PTR [EAX], XMM2, 0x1
    encode32_helper3(Mnemonic::VPEXTRB, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x14, 0x10, 0x01]); // VPEXTRB BYTE PTR [EAX], XMM2, 0x1
    encode32_helper3(Mnemonic::VPEXTRW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x15, 0x10, 0x01]); // VPEXTRW WORD PTR [EAX], XMM2, 0x1
    encode32_helper3(Mnemonic::VPEXTRD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::XMM2), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x79, 0x16, 0x10, 0x01]); // VPEXTRD DWORD PTR [EAX], XMM2, 0x1
}

#[test]
fn instr_phadd() {
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x38, 0x01, 0xCA]); // PHADDW MM1, MM2
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x01, 0x08]); // PHADDW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x01, 0xCA]); // PHADDW XMM1, XMM2
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x01, 0x08]); // PHADDW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x38, 0x02, 0xCA]); // PHADDD MM1, MM2
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x02, 0x08]); // PHADDD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x02, 0xCA]); // PHADDD XMM1, XMM2
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x02, 0x08]); // PHADDD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x01, 0xCB]); // VPHADDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x01, 0x08]); // VPHADDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x02, 0xCB]); // VPHADDD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x02, 0x08]); // VPHADDD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x01, 0xCB]); // VPHADDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x01, 0x08]); // VPHADDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x02, 0xCB]); // VPHADDD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x02, 0x08]); // VPHADDD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_phaddsw() {
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x38, 0x01, 0xCA]); // PHADDW MM1, MM2
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x01, 0x08]); // PHADDW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x01, 0xCA]); // PHADDW XMM1, XMM2
    encode32_helper2(Mnemonic::PHADDW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x01, 0x08]); // PHADDW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::MM1), Operand::Direct(Reg::MM2), &vec![0x0F, 0x38, 0x02, 0xCA]); // PHADDD MM1, MM2
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x02, 0x08]); // PHADDD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x02, 0xCA]); // PHADDD XMM1, XMM2
    encode32_helper2(Mnemonic::PHADDD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x02, 0x08]); // PHADDD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x01, 0xCB]); // VPHADDW XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x01, 0x08]); // VPHADDW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::XMM3), &vec![0xC4, 0xE2, 0x69, 0x02, 0xCB]); // VPHADDD XMM1, XMM2, XMM3
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x02, 0x08]); // VPHADDD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x01, 0xCB]); // VPHADDW YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPHADDW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x01, 0x08]); // VPHADDW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Direct(Reg::YMM3), &vec![0xC4, 0xE2, 0x6D, 0x02, 0xCB]); // VPHADDD YMM1, YMM2, YMM3
    encode32_helper3(Mnemonic::VPHADDD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x02, 0x08]); // VPHADDD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_phminposuw() {
    encode32_helper2(Mnemonic::PHMINPOSUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x41, 0xCA]); // PHMINPOSUW XMM1, XMM2
    encode32_helper2(Mnemonic::PHMINPOSUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), &vec![0x66, 0x0F, 0x38, 0x41, 0xCA]); // PHMINPOSUW XMM1, XMM2
    encode32_helper2(Mnemonic::VPHMINPOSUW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x79, 0x41, 0x08]); // VPHMINPOSUW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPHMINPOSUW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x79, 0x41, 0x08]); // VPHMINPOSUW XMM1, XMMWORD PTR [EAX]
}

#[test]
fn instr_phsub() {
    encode32_helper2(Mnemonic::PHSUBW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x05, 0x08]); // PHSUBW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x05, 0x08]); // PHSUBW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x06, 0x08]); // PHSUBD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x06, 0x08]); // PHSUBD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x05, 0x08]); // VPHSUBW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x06, 0x08]); // VPHSUBD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x05, 0x08]); // VPHSUBW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x06, 0x08]); // VPHSUBD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_phsubsw() {
    encode32_helper2(Mnemonic::PHSUBW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x05, 0x08]); // PHSUBW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x05, 0x08]); // PHSUBW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x06, 0x08]); // PHSUBD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PHSUBD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x06, 0x08]); // PHSUBD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x05, 0x08]); // VPHSUBW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x06, 0x08]); // VPHSUBD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x05, 0x08]); // VPHSUBW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPHSUBD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x06, 0x08]); // VPHSUBD YMM1, YMM2, YMMWORD PTR [EAX]
}

#[test]
fn instr_pinstr() {
    encode32_helper3(Mnemonic::PINSRB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0x3A, 0x20, 0xC8, 0x01]); // PINSRB XMM1, EAX, 0x1
    encode32_helper3(Mnemonic::PINSRW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0xC4, 0xC8, 0x01]); // PINSRW XMM1, EAX, 0x1
    encode32_helper3(Mnemonic::PINSRD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0x66, 0x0F, 0x3A, 0x22, 0xC8, 0x01]); // PINSRD XMM1, EAX, 0x1
    encode32_helper4(Mnemonic::VPINSRB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x69, 0x20, 0xC8, 0x01]); // VPINSRB XMM1, XMM2, EAX, 0x1
    encode32_helper4(Mnemonic::VPINSRW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0xC5, 0xE9, 0xC4, 0xC8, 0x01]); // VPINSRW XMM1, XMM2, EAX, 0x1
    encode32_helper4(Mnemonic::VPINSRD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Direct(Reg::EAX), Operand::Literal8(0x1), &vec![0xC4, 0xE3, 0x69, 0x22, 0xC8, 0x01]); // VPINSRD XMM1, XMM2, EAX, 0x1
}

#[test]
fn instr_pmaddubsw() {
    encode32_helper2(Mnemonic::PMADDUBSW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0x38, 0x04, 0x08]); // PMADDUBSW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMADDUBSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x04, 0x08]); // PMADDUBSW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDUBSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x04, 0x08]); // VPMADDUBSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDUBSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x04, 0x08]); // VPMADDUBSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDUBSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0A, 0x04, 0x08]); // VPMADDUBSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDUBSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2A, 0x04, 0x08]); // VPMADDUBSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDUBSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x4A, 0x04, 0x08]); // VPMADDUBSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmaddwd() {
    encode32_helper2(Mnemonic::PMADDWD, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xF5, 0x08]); // PMADDWD MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMADDWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xF5, 0x08]); // PMADDWD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDWD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xF5, 0x08]); // VPMADDWD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDWD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xF5, 0x08]); // VPMADDWD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDWD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xF5, 0x08]); // VPMADDWD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDWD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xF5, 0x08]); // VPMADDWD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMADDWD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xF5, 0x08]); // VPMADDWD ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmaxs() {
    encode32_helper2(Mnemonic::PMAXSW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xEE, 0x08]); // PMAXSW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXSB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3C, 0x08]); // PMAXSB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xEE, 0x08]); // PMAXSW XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3D, 0x08]); // PMAXSD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3C, 0x08]); // VPMAXSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xEE, 0x08]); // VPMAXSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3D, 0x08]); // VPMAXSD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3C, 0x08]); // VPMAXSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xEE, 0x08]); // VPMAXSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3D, 0x08]); // VPMAXSD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSB, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0E, 0x3C, 0x08]); // VPMAXSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSB, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2E, 0x3C, 0x08]); // VPMAXSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSB, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x4E, 0x3C, 0x08]); // VPMAXSB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0E, 0xEE, 0x08]); // VPMAXSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2E, 0xEE, 0x08]); // VPMAXSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4E, 0xEE, 0x08]); // VPMAXSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0E, 0x3D, 0x08]); // VPMAXSD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2E, 0x3D, 0x08]); // VPMAXSD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x5E, 0x3D, 0x08]); // VPMAXSD ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper3(Mnemonic::VPMAXSQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x0E, 0x3D, 0x08]); // VPMAXSQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x2E, 0x3D, 0x08]); // VPMAXSQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXSQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K6), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x5E, 0x3D, 0x08]); // VPMAXSQ ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_pmaxu() {
    encode32_helper2(Mnemonic::PMAXUB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xDE, 0x08]); // PMAXUB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXUB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xDE, 0x08]); // PMAXUB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXUW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3E, 0x08]); // PMAXUW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDE, 0x08]); // VPMAXUB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xDE, 0x08]); // VPMAXUB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3E, 0x08]); // VPMAXUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3E, 0x08]); // VPMAXUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDE, 0x08]); // VPMAXUB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xDE, 0x08]); // VPMAXUB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUB, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x48, 0xDE, 0x08]); // VPMAXUB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3E, 0x08]); // VPMAXUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3E, 0x08]); // VPMAXUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUW, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x48, 0x3E, 0x08]); // VPMAXUW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMAXUD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3F, 0x08]); // PMAXUD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3F, 0x08]); // VPMAXUD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3F, 0x08]); // VPMAXUD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0B, 0x3F, 0x08]); // VPMAXUD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2B, 0x3F, 0x08]); // VPMAXUD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x5B, 0x3F, 0x08]); // VPMAXUD ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper3(Mnemonic::VPMAXUQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x0B, 0x3F, 0x08]); // VPMAXUQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x2B, 0x3F, 0x08]); // VPMAXUQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMAXUQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K3), None), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x5B, 0x3F, 0x08]); // VPMAXUQ ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_pmins() {
    encode32_helper2(Mnemonic::PMINSW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xEA, 0x08]); // PMINSW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMINSB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x38, 0x08]); // PMINSB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMINSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xEA, 0x08]); // PMINSW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x38, 0x08]); // VPMINSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x38, 0x08]); // VPMINSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xEA, 0x08]); // VPMINSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xEA, 0x08]); // VPMINSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x38, 0x08]); // VPMINSB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x38, 0x08]); // VPMINSB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSB, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x48, 0x38, 0x08]); // VPMINSB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xEA, 0x08]); // VPMINSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xEA, 0x08]); // VPMINSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSW, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x48, 0xEA, 0x08]); // VPMINSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMINSD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x39, 0x08]); // PMINSD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x39, 0x08]); // VPMINSD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x39, 0x08]); // VPMINSD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x18, 0x39, 0x08]); // VPMINSD XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x38, 0x39, 0x08]); // VPMINSD YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x58, 0x39, 0x08]); // VPMINSD ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x18, 0x39, 0x08]); // VPMINSQ XMM1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x38, 0x39, 0x08]); // VPMINSQ YMM1, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x58, 0x39, 0x08]); // VPMINSQ ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_pminu() {
    encode32_helper2(Mnemonic::PMINUD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3B, 0x08]); // PMINUD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x39, 0x08]); // VPMINSD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x39, 0x08]); // VPMINSD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x18, 0x39, 0x08]); // VPMINSD XMM1, XMM2, DWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x38, 0x39, 0x08]); // VPMINSD YMM1, YMM2, DWORD PTR [EAX]{1to8}
    encode32_helper3(Mnemonic::VPMINSD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To16, Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x6D, 0x58, 0x39, 0x08]); // VPMINSD ZMM1, ZMM2, DWORD PTR [EAX]{1to16}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x18, 0x39, 0x08]); // VPMINSQ XMM1, XMM2, QWORD PTR [EAX]{1to2}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x38, 0x39, 0x08]); // VPMINSQ YMM1, YMM2, QWORD PTR [EAX]{1to4}
    encode32_helper3(Mnemonic::VPMINSQ, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x58, 0x39, 0x08]); // VPMINSQ ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
    encode32_helper2(Mnemonic::PMINUB, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xDA, 0x08]); // PMINUB MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMINUB, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xDA, 0x08]); // PMINUB XMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMINUW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x3A, 0x08]); // PMINUW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDA, 0x08]); // VPMINUB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xDA, 0x08]); // VPMINUB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3A, 0x08]); // VPMINUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3A, 0x08]); // VPMINUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUB, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xDA, 0x08]); // VPMINUB XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUB, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xDA, 0x08]); // VPMINUB YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUB, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x48, 0xDA, 0x08]); // VPMINUB ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x3A, 0x08]); // VPMINUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x3A, 0x08]); // VPMINUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMINUW, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x48, 0x3A, 0x08]); // VPMINUW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmovmskb() {
    encode32_helper2(Mnemonic::PMOVMSKB, Operand::Direct(Reg::EAX), Operand::Direct(Reg::MM1), &vec![0x0F, 0xD7, 0xC1]); // PMOVMSKB EAX, MM1
    encode32_helper2(Mnemonic::PMOVMSKB, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0x66, 0x0F, 0xD7, 0xC1]); // PMOVMSKB EAX, XMM1
    encode32_helper2(Mnemonic::VPMOVMSKB, Operand::Direct(Reg::EAX), Operand::Direct(Reg::XMM1), &vec![0xC5, 0xF9, 0xD7, 0xC1]); // VPMOVMSKB EAX, XMM1
    encode32_helper2(Mnemonic::VPMOVMSKB, Operand::Direct(Reg::EAX), Operand::Direct(Reg::YMM1), &vec![0xC5, 0xFD, 0xD7, 0xC1]); // VPMOVMSKB EAX, YMM1
}

#[test]
fn instr_pmovsx() {
    encode32_helper2(Mnemonic::PMOVSXBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x20, 0x08]); // PMOVSXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVSXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x23, 0x08]); // PMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVSXBQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0x38, 0x22, 0x08]); // PMOVSXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVSXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x23, 0x08]); // PMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVSXWQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x0F, 0x38, 0x24, 0x08]); // PMOVSXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVSXDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x25, 0x08]); // PMOVSXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x20, 0x08]); // VPMOVSXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x23, 0x08]); // VPMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xC4, 0xE2, 0x79, 0x22, 0x08]); // VPMOVSXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x23, 0x08]); // VPMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x79, 0x24, 0x08]); // VPMOVSXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x25, 0x08]); // VPMOVSXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBW, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x20, 0x08]); // VPMOVSXBW YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x23, 0x08]); // VPMOVSXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x7D, 0x22, 0x08]); // VPMOVSXBQ YMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x23, 0x08]); // VPMOVSXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x7D, 0x24, 0x08]); // VPMOVSXWQ YMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXDQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x25, 0x08]); // VPMOVSXDQ YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x20, 0x08]); // VPMOVSXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x23, 0x08]); // VPMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x22, 0x08]); // VPMOVSXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x23, 0x08]); // VPMOVSXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x24, 0x08]); // VPMOVSXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXDQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x25, 0x08]); // VPMOVSXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x20, 0x08]); // VPMOVSXBW YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x23, 0x08]); // VPMOVSXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXBQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x22, 0x08]); // VPMOVSXBQ YMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x23, 0x08]); // VPMOVSXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXWQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x24, 0x08]); // VPMOVSXWQ YMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVSXDQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x25, 0x08]); // VPMOVSXDQ YMM1, XMMWORD PTR [EAX]
}

#[test]
fn instr_pmovzx() {
    encode32_helper2(Mnemonic::PMOVZXBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x30, 0x08]); // PMOVZXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVZXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x33, 0x08]); // PMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVZXBQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0x38, 0x32, 0x08]); // PMOVZXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVZXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x33, 0x08]); // PMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVZXWQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x0F, 0x38, 0x34, 0x08]); // PMOVZXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMOVZXDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x66, 0x0F, 0x38, 0x35, 0x08]); // PMOVZXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x30, 0x08]); // VPMOVZXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x33, 0x08]); // VPMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xC4, 0xE2, 0x79, 0x32, 0x08]); // VPMOVZXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x33, 0x08]); // VPMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x79, 0x34, 0x08]); // VPMOVZXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x79, 0x35, 0x08]); // VPMOVZXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBW, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x30, 0x08]); // VPMOVZXBW YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x33, 0x08]); // VPMOVZXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xC4, 0xE2, 0x7D, 0x32, 0x08]); // VPMOVZXBQ YMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x33, 0x08]); // VPMOVZXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xC4, 0xE2, 0x7D, 0x34, 0x08]); // VPMOVZXWQ YMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXDQ, Operand::Direct(Reg::YMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x7D, 0x35, 0x08]); // VPMOVZXDQ YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x30, 0x08]); // VPMOVZXBW XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x33, 0x08]); // VPMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x32, 0x08]); // VPMOVZXBQ XMM1, WORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x33, 0x08]); // VPMOVZXWD XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x34, 0x08]); // VPMOVZXWQ XMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXDQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x0A, 0x35, 0x08]); // VPMOVZXDQ XMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x30, 0x08]); // VPMOVZXBW YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x33, 0x08]); // VPMOVZXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXBQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x32, 0x08]); // VPMOVZXBQ YMM1, DWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x33, 0x08]); // VPMOVZXWD YMM1, XMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXWQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x34, 0x08]); // VPMOVZXWQ YMM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::VPMOVZXDQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x7D, 0x2A, 0x35, 0x08]); // VPMOVZXDQ YMM1, XMMWORD PTR [EAX]
}

#[test]
fn instr_pmuldq() {
    encode32_helper2(Mnemonic::PMULDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x28, 0x08]); // PMULDQ XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x28, 0x08]); // VPMULDQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULDQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x28, 0x08]); // VPMULDQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To2, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x18, 0x28, 0x08]); // VPMULDQ XMM1, XMM2, QWORD PTR [EAX]{1to2}
encode32_helper3(Mnemonic::VPMULDQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To4, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x38, 0x28, 0x08]); // VPMULDQ YMM1, YMM2, QWORD PTR [EAX]{1to4}
encode32_helper3(Mnemonic::VPMULDQ, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::AVXBroadcastIndirect(BroadcastMode::Broadcast1To8, Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0xF2, 0xED, 0x58, 0x28, 0x08]); // VPMULDQ ZMM1, ZMM2, QWORD PTR [EAX]{1to8}
}

#[test]
fn instr_pmulhrsw() {
    encode32_helper2(Mnemonic::PMULHRSW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x0B, 0x08]); // PMULHRSW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHRSW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x0B, 0x08]); // VPMULHRSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHRSW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x0B, 0x08]); // VPMULHRSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHRSW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0A, 0x0B, 0x08]); // VPMULHRSW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHRSW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2A, 0x0B, 0x08]); // VPMULHRSW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHRSW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x4A, 0x0B, 0x08]); // VPMULHRSW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}
#[test]
fn instr_pmulhuw() {
    encode32_helper2(Mnemonic::PMULHUW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xE4, 0x08]); // PMULHUW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHUW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xE4, 0x08]); // VPMULHUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHUW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE4, 0x08]); // VPMULHUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHUW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xE4, 0x08]); // VPMULHUW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHUW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xE4, 0x08]); // VPMULHUW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHUW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xE4, 0x08]); // VPMULHUW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmulhw() {
    encode32_helper2(Mnemonic::PMULHW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xE5, 0x08]); // PMULHW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xE5, 0x08]); // VPMULHW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xE5, 0x08]); // VPMULHW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xE5, 0x08]); // VPMULHW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xE5, 0x08]); // VPMULHW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULHW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xE5, 0x08]); // VPMULHW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmull() {
    encode32_helper2(Mnemonic::PMULLD, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0x38, 0x40, 0x08]); // PMULLD XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC4, 0xE2, 0x69, 0x40, 0x08]); // VPMULLD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC4, 0xE2, 0x6D, 0x40, 0x08]); // VPMULLD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLD, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0x6D, 0x0A, 0x40, 0x08]); // VPMULLD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLD, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0x6D, 0x2A, 0x40, 0x08]); // VPMULLD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLD, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0x6D, 0x4A, 0x40, 0x08]); // VPMULLD ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x08, 0x40, 0x08]); // VPMULLQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x28, 0x40, 0x08]); // VPMULLQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF2, 0xED, 0x0A, 0x40, 0x08]); // VPMULLQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF2, 0xED, 0x2A, 0x40, 0x08]); // VPMULLQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF2, 0xED, 0x4A, 0x40, 0x08]); // VPMULLQ ZMM1, ZMM2, ZMMWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMULLW, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xD5, 0x08]); // PMULLW MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMULLW, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xD5, 0x08]); // PMULLW XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLW, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xD5, 0x08]); // VPMULLW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLW, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xD5, 0x08]); // VPMULLW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLW, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0x6D, 0x0A, 0xD5, 0x08]); // VPMULLW XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLW, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0x6D, 0x2A, 0xD5, 0x08]); // VPMULLW YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULLW, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0x6D, 0x4A, 0xD5, 0x08]); // VPMULLW ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pmuludq() {
    encode32_helper2(Mnemonic::PMULUDQ, Operand::Direct(Reg::MM1), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x0F, 0xF4, 0x08]); // PMULUDQ MM1, QWORD PTR [EAX]
    encode32_helper2(Mnemonic::PMULUDQ, Operand::Direct(Reg::XMM1), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x66, 0x0F, 0xF4, 0x08]); // PMULUDQ XMM1, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULUDQ, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0xC5, 0xE9, 0xF4, 0x08]); // VPMULUDQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULUDQ, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0xC5, 0xED, 0xF4, 0x08]); // VPMULUDQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULUDQ, Operand::AVXDestination(Reg::XMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::XMMword), None), &vec![0x62, 0xF1, 0xED, 0x0A, 0xF4, 0x08]); // VPMULUDQ XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULUDQ, Operand::AVXDestination(Reg::YMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::YMMword), None), &vec![0x62, 0xF1, 0xED, 0x2A, 0xF4, 0x08]); // VPMULUDQ YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VPMULUDQ, Operand::AVXDestination(Reg::ZMM1, Some(MaskReg::K2), None), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, Some(OperandSize::ZMMword), None), &vec![0x62, 0xF1, 0xED, 0x4A, 0xF4, 0x08]); // VPMULUDQ ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn instr_pop() {
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::AX), &vec![0x66, 0x58]); // POP AX
    encode32_helper1(Mnemonic::POP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x8F, 0x00]); // POP WORD PTR [EAX]
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::EAX), &vec![0x58]); // POP EAX
    encode32_helper1(Mnemonic::POP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x8F, 0x00]); // POP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::DS), &vec![0x1F]); // POP DS
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::ES), &vec![0x07]); // POP ES
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::SS), &vec![0x17]); // POP SS
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::FS), &vec![0x0F, 0xA1]); // POP FS
    encode32_helper1(Mnemonic::POP, Operand::Direct(Reg::GS), &vec![0x0F, 0xA9]); // POP GS
}

#[test]
fn instr_popa() {
    encode32_helper0(Mnemonic::POPA,  &vec![0x61]); // POPA
}

#[test]
fn instr_popcnt() {
    encode32_helper2(Mnemonic::POPCNT, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF3, 0x0F, 0xB8, 0x00]); // POPCNT AX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::POPCNT, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF3, 0x0F, 0xB8, 0x00]); // POPCNT EAX, DWORD PTR [EAX]
}
