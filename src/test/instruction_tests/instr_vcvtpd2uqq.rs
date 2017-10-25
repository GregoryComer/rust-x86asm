use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 121, 222], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 257490578, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 121, 36, 149, 146, 254, 88, 15], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 141, 121, 248], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 379868812, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 121, 152, 140, 86, 164, 22], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 121, 238], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 2054685879, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 121, 132, 200, 183, 4, 120, 122], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 170, 121, 255], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1209499223, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 121, 12, 141, 87, 126, 23, 72], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 188, 121, 241], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 121, 46], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 253, 217, 121, 217], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM21)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 205, 121, 40], OperandSize::Qword)
}

