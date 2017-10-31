use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 121, 230], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 121, 44, 115], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 252, 141, 121, 252], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 252, 140, 121, 60, 179], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 170, 121, 195], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 1268403744, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 175, 121, 138, 32, 78, 154, 75], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 252, 169, 121, 193], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1531834059, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 252, 174, 121, 12, 69, 203, 238, 77, 91], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 217, 121, 240], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 203, 121, 36, 78], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 252, 251, 121, 199], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 2117893545, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 252, 201, 121, 164, 64, 169, 125, 60, 126], OperandSize::Qword)
}

