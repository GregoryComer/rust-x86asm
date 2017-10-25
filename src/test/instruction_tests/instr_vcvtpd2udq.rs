use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 121, 230], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2006221188, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 143, 121, 4, 189, 132, 129, 148, 119], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 252, 141, 121, 234], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 1424182369, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 143, 121, 161, 97, 76, 227, 84], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 171, 121, 250], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 175, 121, 28, 183], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 252, 171, 121, 232], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM29)), operand2: Some(IndirectDisplaced(RAX, 749203244, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 252, 171, 121, 168, 44, 239, 167, 44], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 186, 121, 250], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EBX, 890289909, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 204, 121, 179, 245, 190, 16, 53], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 252, 221, 121, 234], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RBX, 1762678610, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 252, 201, 121, 155, 82, 87, 16, 105], OperandSize::Qword)
}

