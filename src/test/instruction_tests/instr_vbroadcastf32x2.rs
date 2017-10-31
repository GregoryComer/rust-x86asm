use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 25, 224], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 14452241, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 25, 60, 245, 17, 134, 220, 0], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 125, 169, 25, 223], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1836298233, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 170, 25, 12, 189, 249, 175, 115, 109], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 25, 198], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 553341185, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 25, 180, 191, 1, 81, 251, 32], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 201, 25, 246], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 25, 28, 138], OperandSize::Qword)
}

