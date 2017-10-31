use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 121, 205], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1836772540, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 121, 140, 207, 188, 236, 122, 109], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 141, 121, 247], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 1471323682, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 121, 137, 34, 158, 178, 87], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 121, 213], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDX, 1948751860, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 121, 170, 244, 151, 39, 116], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 124, 174, 121, 219], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 875344295, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 174, 121, 148, 83, 167, 177, 44, 52], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 255, 121, 209], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 121, 20, 73], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 124, 159, 121, 198], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 206, 121, 44, 190], OperandSize::Qword)
}

