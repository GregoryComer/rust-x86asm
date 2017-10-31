use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 252], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 4, 74], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 194], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1243721555, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 132, 138, 83, 175, 33, 74], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1534006315, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 140, 178, 43, 20, 111, 91], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 238], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 52, 187], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 33, 209], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 2057195895, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 33, 140, 243, 119, 81, 158, 122], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 125, 141, 33, 239], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1401732484, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 33, 36, 157, 132, 189, 140, 83], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 33, 246], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 33, 28, 222], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 125, 171, 33, 229], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RSI, 1468888584, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 33, 182, 8, 118, 141, 87], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 33, 232], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ECX, 1554372975, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 33, 137, 111, 217, 165, 92], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 125, 207, 33, 212], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 812788722, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 33, 140, 137, 242, 43, 114, 48], OperandSize::Qword)
}

