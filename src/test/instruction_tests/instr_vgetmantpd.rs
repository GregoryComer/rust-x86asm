use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 38, 193, 83], OperandSize::Dword)
}

#[test]
fn vgetmantpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1599187068, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 38, 12, 133, 124, 168, 81, 95, 41], OperandSize::Dword)
}

#[test]
fn vgetmantpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1784374652, Some(OperandSize::Qword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 159, 38, 36, 85, 124, 101, 91, 106, 111], OperandSize::Dword)
}

#[test]
fn vgetmantpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 19, 253, 142, 38, 207, 10], OperandSize::Qword)
}

#[test]
fn vgetmantpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 38, 23, 124], OperandSize::Qword)
}

#[test]
fn vgetmantpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM11)), operand2: Some(IndirectDisplaced(RSI, 410974995, Some(OperandSize::Qword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 155, 38, 158, 19, 251, 126, 24, 2], OperandSize::Qword)
}

#[test]
fn vgetmantpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 38, 230, 28], OperandSize::Dword)
}

#[test]
fn vgetmantpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 38, 30, 62], OperandSize::Dword)
}

#[test]
fn vgetmantpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 172259999, Some(OperandSize::Qword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 38, 156, 153, 159, 122, 68, 10, 79], OperandSize::Dword)
}

#[test]
fn vgetmantpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 253, 171, 38, 229, 64], OperandSize::Qword)
}

#[test]
fn vgetmantpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectDisplaced(RCX, 522558410, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 253, 170, 38, 145, 202, 155, 37, 31, 47], OperandSize::Qword)
}

#[test]
fn vgetmantpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1766431593, Some(OperandSize::Qword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 253, 191, 38, 172, 86, 105, 155, 73, 105, 6], OperandSize::Qword)
}

#[test]
fn vgetmantpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 158, 38, 207, 50], OperandSize::Dword)
}

#[test]
fn vgetmantpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 422832491, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 38, 167, 107, 233, 51, 25, 58], OperandSize::Dword)
}

#[test]
fn vgetmantpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDI, 856219346, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 38, 151, 210, 222, 8, 51, 98], OperandSize::Dword)
}

#[test]
fn vgetmantpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 253, 153, 38, 238, 97], OperandSize::Qword)
}

#[test]
fn vgetmantpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 204, 38, 20, 82, 66], OperandSize::Qword)
}

#[test]
fn vgetmantpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectDisplaced(RDX, 1318282091, Some(OperandSize::Qword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 221, 38, 154, 107, 99, 147, 78, 12], OperandSize::Qword)
}

