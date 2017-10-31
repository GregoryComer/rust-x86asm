use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 195], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1539324324, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 20, 189, 164, 57, 192, 91], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 239], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 56], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 193], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1727202923, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 4, 213, 107, 6, 243, 102], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 254], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1185643626, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 164, 210, 106, 124, 171, 70], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 36, 248], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 543107339, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 36, 172, 120, 11, 41, 95, 32], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 36, 201], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 208753004, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 36, 52, 205, 108, 81, 113, 12], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 36, 201], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1927953285, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 36, 4, 181, 133, 59, 234, 114], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 36, 219], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 987262150, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 36, 60, 117, 198, 108, 216, 58], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 36, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 172882668, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 36, 12, 133, 236, 250, 77, 10], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 36, 212], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectDisplaced(RCX, 117695057, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 36, 129, 81, 226, 3, 7], OperandSize::Qword)
}

