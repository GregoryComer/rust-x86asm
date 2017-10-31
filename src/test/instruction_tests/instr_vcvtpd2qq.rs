use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 123, 231], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 1386561329, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 123, 139, 49, 63, 165, 82], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 140, 123, 239], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1504884388, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 137, 123, 132, 130, 164, 182, 178, 89], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 123, 225], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1869314550, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 123, 28, 157, 246, 121, 107, 111], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 253, 171, 123, 254], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1626460997, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 173, 123, 28, 85, 69, 211, 241, 96], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 255, 123, 193], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDX, 1642606507, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 123, 170, 171, 47, 232, 97], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 253, 188, 123, 209], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1267916911, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 253, 204, 123, 148, 206, 111, 224, 146, 75], OperandSize::Qword)
}

