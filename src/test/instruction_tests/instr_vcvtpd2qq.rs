use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 123, 253], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 123, 44, 206], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 253, 142, 123, 206], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 123, 60, 119], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 123, 218], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 123, 47], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 253, 175, 123, 233], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1390954009, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 173, 123, 20, 181, 25, 70, 232, 82], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 123, 220], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 643490724, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 123, 52, 213, 164, 227, 90, 38], OperandSize::Dword)
}

#[test]
fn vcvtpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 253, 187, 123, 242], OperandSize::Qword)
}

#[test]
fn vcvtpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 653976893, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 201, 123, 12, 85, 61, 229, 250, 38], OperandSize::Qword)
}

