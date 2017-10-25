use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 68, 234], OperandSize::Dword)
}

#[test]
fn vplzcntd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 1864331162, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 68, 134, 154, 111, 31, 111], OperandSize::Dword)
}

#[test]
fn vplzcntd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 68, 28, 158], OperandSize::Dword)
}

#[test]
fn vplzcntd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 139, 68, 212], OperandSize::Qword)
}

#[test]
fn vplzcntd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1035602099, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 143, 68, 28, 245, 179, 8, 186, 61], OperandSize::Qword)
}

#[test]
fn vplzcntd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1539478142, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 157, 68, 172, 198, 126, 146, 194, 91], OperandSize::Qword)
}

#[test]
fn vplzcntd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 68, 252], OperandSize::Dword)
}

#[test]
fn vplzcntd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 752846119, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 68, 175, 39, 133, 223, 44], OperandSize::Dword)
}

#[test]
fn vplzcntd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 991675847, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 68, 132, 120, 199, 197, 27, 59], OperandSize::Dword)
}

#[test]
fn vplzcntd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 169, 68, 225], OperandSize::Qword)
}

#[test]
fn vplzcntd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM13)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 173, 68, 46], OperandSize::Qword)
}

#[test]
fn vplzcntd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 68, 12, 190], OperandSize::Qword)
}

#[test]
fn vplzcntd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 68, 248], OperandSize::Dword)
}

#[test]
fn vplzcntd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 609173059, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 68, 52, 157, 67, 62, 79, 36], OperandSize::Dword)
}

#[test]
fn vplzcntd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 2055117183, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 68, 172, 202, 127, 153, 126, 122], OperandSize::Dword)
}

#[test]
fn vplzcntd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 68, 222], OperandSize::Qword)
}

#[test]
fn vplzcntd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 438859344, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 68, 36, 149, 80, 118, 40, 26], OperandSize::Qword)
}

#[test]
fn vplzcntd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 221, 68, 28, 206], OperandSize::Qword)
}

