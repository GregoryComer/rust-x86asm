use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 121, 195], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 121, 52, 195], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 252, 142, 121, 237], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 121, 39], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 121, 230], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 313282343, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 173, 121, 188, 151, 39, 79, 172, 18], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 252, 174, 121, 230], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 252, 174, 121, 44, 192], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 252, 121, 219], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 204, 121, 58], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 252, 186, 121, 244], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 643010112, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 252, 203, 121, 60, 141, 64, 142, 83, 38], OperandSize::Qword)
}

