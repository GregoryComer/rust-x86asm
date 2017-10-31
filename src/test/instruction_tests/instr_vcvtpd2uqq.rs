use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 121, 223], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 121, 19], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 253, 143, 121, 217], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 142, 121, 20, 72], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 121, 204], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1340401199, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 121, 44, 69, 47, 230, 228, 79], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 171, 121, 206], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 829849733, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 174, 121, 28, 157, 133, 128, 118, 49], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 154, 121, 240], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 557782539, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 121, 180, 136, 11, 22, 63, 33], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 253, 121, 240], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 253, 204, 121, 28, 208], OperandSize::Qword)
}

