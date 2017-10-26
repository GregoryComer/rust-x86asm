use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 123, 198], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 123, 0], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 253, 143, 123, 250], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 123, 60, 88], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 123, 223], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 123, 11], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 174, 123, 229], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectDisplaced(RBX, 1891811227, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 173, 123, 163, 155, 191, 194, 112], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 221, 123, 238], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 123, 54], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 253, 221, 123, 244], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectDisplaced(RCX, 512526368, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 205, 123, 177, 32, 136, 140, 30], OperandSize::Qword)
}

