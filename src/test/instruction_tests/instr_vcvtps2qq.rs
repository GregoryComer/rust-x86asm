use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 123, 247], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 2073365444, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 123, 132, 75, 196, 11, 149, 123], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 125, 138, 123, 195], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectDisplaced(RDX, 1694616928, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 137, 123, 146, 96, 205, 1, 101], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 123, 203], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 123, 3], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 123, 220], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 171, 123, 20, 182], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 154, 123, 211], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 354903404, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 123, 20, 213, 108, 101, 39, 21], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 187, 123, 208], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 84928216, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 125, 202, 123, 180, 79, 216, 230, 15, 5], OperandSize::Qword)
}

