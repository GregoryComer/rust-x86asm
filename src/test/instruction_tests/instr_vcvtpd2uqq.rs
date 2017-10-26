use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 121, 239], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1785489513, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 121, 154, 105, 104, 108, 106], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 253, 139, 121, 207], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM29)), operand2: Some(IndirectDisplaced(RSI, 366087586, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 142, 121, 174, 162, 13, 210, 21], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 121, 202], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2123825376, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 121, 36, 189, 224, 0, 151, 126], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 253, 171, 121, 192], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 169, 121, 8], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 188, 121, 217], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 121, 44, 152], OperandSize::Dword)
}

#[test]
fn vcvtpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 253, 190, 121, 237], OperandSize::Qword)
}

#[test]
fn vcvtpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 108752154, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 121, 164, 185, 26, 109, 123, 6], OperandSize::Qword)
}

