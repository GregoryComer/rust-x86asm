use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 120, 194], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 283223872, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 120, 172, 82, 64, 167, 225, 16], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 125, 137, 120, 194], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1721806153, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 142, 120, 60, 197, 73, 173, 160, 102], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 120, 224], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EDI, 1192573007, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 120, 135, 79, 56, 21, 71], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 125, 169, 120, 207], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 334656343, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 120, 44, 69, 87, 115, 242, 19], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 153, 120, 217], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDX, 1322012578, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 120, 170, 162, 79, 204, 78], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 125, 157, 120, 240], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1660234932, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 120, 44, 77, 180, 44, 245, 98], OperandSize::Qword)
}

