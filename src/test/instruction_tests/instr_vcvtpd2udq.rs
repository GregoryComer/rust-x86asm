use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 121, 239], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 121, 52, 71], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 252, 138, 121, 229], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 358017716, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 121, 36, 69, 180, 234, 86, 21], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 175, 121, 240], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1833102068, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 169, 121, 164, 81, 244, 234, 66, 109], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 252, 172, 121, 212], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 175, 121, 12, 121], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 255, 121, 246], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 241753683, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 205, 121, 156, 83, 83, 222, 104, 14], OperandSize::Dword)
}

#[test]
fn vcvtpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 252, 218, 121, 240], OperandSize::Qword)
}

#[test]
fn vcvtpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 252, 202, 121, 12, 89], OperandSize::Qword)
}

