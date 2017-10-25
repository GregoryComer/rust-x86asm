use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(BP, 17348, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 134, 196, 67], OperandSize::Word)
}

#[test]
fn fadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 594038661, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 132, 118, 133, 79, 104, 35], OperandSize::Dword)
}

#[test]
fn fadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledDisplaced(RSI, Two, 785123990, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 4, 117, 150, 10, 204, 46], OperandSize::Qword)
}

#[test]
fn fadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 198], OperandSize::Word)
}

#[test]
fn fadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 196], OperandSize::Dword)
}

#[test]
fn fadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 195], OperandSize::Qword)
}

#[test]
fn fadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 24235, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 130, 171, 94], OperandSize::Word)
}

#[test]
fn fadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 2], OperandSize::Dword)
}

#[test]
fn fadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledDisplaced(RBX, Two, 250166614, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 4, 93, 86, 61, 233, 14], OperandSize::Qword)
}

#[test]
fn fadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 194], OperandSize::Word)
}

#[test]
fn fadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 193], OperandSize::Dword)
}

#[test]
fn fadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 196], OperandSize::Qword)
}

