use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 123, 238], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 123, 3], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 125, 138, 123, 223], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 125, 139, 123, 44, 114], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 123, 250], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 123, 20, 119], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 123, 245], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM31)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 172, 123, 57], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 251, 123, 250], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 123, 48], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 125, 223, 123, 230], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(RSI, 2028039502, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 123, 190, 78, 109, 225, 120], OperandSize::Qword)
}

