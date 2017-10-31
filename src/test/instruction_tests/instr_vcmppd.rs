use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 194, 227, 70], OperandSize::Dword)
}

#[test]
fn vcmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1587845521, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 194, 4, 125, 145, 153, 164, 94, 91], OperandSize::Dword)
}

#[test]
fn vcmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 194, 217, 5], OperandSize::Qword)
}

#[test]
fn vcmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 997461056, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 194, 156, 210, 64, 12, 116, 59, 37], OperandSize::Qword)
}

#[test]
fn vcmppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 194, 220, 6], OperandSize::Dword)
}

#[test]
fn vcmppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 194, 63, 126], OperandSize::Dword)
}

#[test]
fn vcmppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 194, 223, 21], OperandSize::Qword)
}

#[test]
fn vcmppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 194, 20, 251, 89], OperandSize::Qword)
}

#[test]
fn vcmppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 12, 194, 233, 102], OperandSize::Dword)
}

#[test]
fn vcmppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 318132397, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 14, 194, 52, 157, 173, 80, 246, 18, 67], OperandSize::Dword)
}

#[test]
fn vcmppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 869858011, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 26, 194, 180, 143, 219, 250, 216, 51, 111], OperandSize::Dword)
}

#[test]
fn vcmppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 12, 194, 209, 32], OperandSize::Qword)
}

#[test]
fn vcmppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 10, 194, 60, 142, 14], OperandSize::Qword)
}

#[test]
fn vcmppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1142664215, Some(OperandSize::Qword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 31, 194, 162, 23, 172, 27, 68, 53], OperandSize::Qword)
}

#[test]
fn vcmppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 47, 194, 251, 83], OperandSize::Dword)
}

#[test]
fn vcmppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 42, 194, 49, 103], OperandSize::Dword)
}

#[test]
fn vcmppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 1244738320, Some(OperandSize::Qword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 61, 194, 175, 16, 51, 49, 74, 58], OperandSize::Dword)
}

#[test]
fn vcmppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 165, 39, 194, 238, 38], OperandSize::Qword)
}

#[test]
fn vcmppd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1010702068, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 165, 33, 194, 148, 81, 244, 22, 62, 60, 56], OperandSize::Qword)
}

#[test]
fn vcmppd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 181, 49, 194, 23, 68], OperandSize::Qword)
}

#[test]
fn vcmppd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 26, 194, 226, 75], OperandSize::Dword)
}

#[test]
fn vcmppd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 76, 194, 11, 79], OperandSize::Dword)
}

#[test]
fn vcmppd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 91, 194, 34, 111], OperandSize::Dword)
}

#[test]
fn vcmppd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 149, 29, 194, 207, 22], OperandSize::Qword)
}

#[test]
fn vcmppd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 951401151, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 78, 194, 52, 149, 191, 58, 181, 56, 114], OperandSize::Qword)
}

#[test]
fn vcmppd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 173, 84, 194, 34, 94], OperandSize::Qword)
}

