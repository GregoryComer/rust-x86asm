use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 237], OperandSize::Dword)
}

#[test]
fn psignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 30], OperandSize::Dword)
}

#[test]
fn psignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 255], OperandSize::Qword)
}

#[test]
fn psignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 56], OperandSize::Qword)
}

#[test]
fn psignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 239], OperandSize::Dword)
}

#[test]
fn psignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 559232164, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 140, 138, 164, 52, 85, 33], OperandSize::Dword)
}

#[test]
fn psignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 212], OperandSize::Qword)
}

#[test]
fn psignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 137683311, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 140, 112, 111, 225, 52, 8], OperandSize::Qword)
}

