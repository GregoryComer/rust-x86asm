use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 10720, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 131, 224, 41], OperandSize::Word)
}

#[test]
fn fadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 0], OperandSize::Dword)
}

#[test]
fn fadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 1], OperandSize::Qword)
}

#[test]
fn fadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 196], OperandSize::Word)
}

#[test]
fn fadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 195], OperandSize::Dword)
}

#[test]
fn fadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 195], OperandSize::Qword)
}

#[test]
fn fadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(DI, 61, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 69, 61], OperandSize::Word)
}

#[test]
fn fadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 0], OperandSize::Dword)
}

#[test]
fn fadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1565065860, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 4, 141, 132, 2, 73, 93], OperandSize::Qword)
}

#[test]
fn fadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 193], OperandSize::Word)
}

#[test]
fn fadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 197], OperandSize::Dword)
}

#[test]
fn fadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 199], OperandSize::Qword)
}

