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
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 1490259834, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 121, 186, 122, 143, 211, 88], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 253, 137, 121, 227], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM14)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 142, 121, 55], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 121, 246], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 121, 41], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 253, 169, 121, 247], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 171, 121, 36, 152], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 187, 121, 232], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 121, 25], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 155, 121, 243], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1064868687, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 205, 121, 148, 137, 79, 155, 120, 63], OperandSize::Qword)
}

