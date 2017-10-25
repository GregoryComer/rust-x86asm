use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 123, 234], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 123, 44, 130], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 253, 139, 123, 225], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectDisplaced(RSI, 950992556, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 141, 123, 166, 172, 254, 174, 56], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 123, 239], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 123, 4, 219], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 253, 170, 123, 234], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RSI, 1577237829, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 174, 123, 166, 69, 189, 2, 94], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 221, 123, 255], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1024029443, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 123, 164, 152, 3, 115, 9, 61], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 253, 221, 123, 243], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectDisplaced(RAX, 2014722592, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 205, 123, 152, 32, 58, 22, 120], OperandSize::Qword)
}

