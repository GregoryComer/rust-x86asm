use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 120, 197], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDI, 413491257, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 120, 143, 57, 96, 165, 24], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 253, 138, 120, 230], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectDisplaced(RDX, 2088850569, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 141, 120, 186, 137, 84, 129, 124], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 120, 234], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1725636143, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 120, 172, 194, 47, 30, 219, 102], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 253, 170, 120, 242], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 120, 43], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 158, 120, 209], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 120, 28, 80], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 253, 159, 120, 231], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectDisplaced(RCX, 1532126144, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 206, 120, 185, 192, 99, 82, 91], OperandSize::Qword)
}

