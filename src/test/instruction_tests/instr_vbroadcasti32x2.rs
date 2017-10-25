use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 89, 226], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 89, 52, 73], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 125, 142, 89, 244], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1469894214, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 138, 89, 132, 240, 70, 206, 156, 87], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 89, 206], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1321018807, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 89, 4, 181, 183, 37, 189, 78], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 169, 89, 255], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1426649418, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 89, 164, 91, 74, 241, 8, 85], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 89, 225], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 306666235, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 89, 132, 193, 251, 90, 71, 18], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 202, 89, 247], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 462132927, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 89, 52, 157, 191, 150, 139, 27], OperandSize::Qword)
}

