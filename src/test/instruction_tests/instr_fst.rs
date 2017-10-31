use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fst_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 176, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 146, 176, 0], OperandSize::Word)
}

#[test]
fn fst_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(EBX, 1729436355, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 147, 195, 26, 21, 103], OperandSize::Dword)
}

#[test]
fn fst_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(RBX, 282115614, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 147, 30, 190, 208, 16], OperandSize::Qword)
}

#[test]
fn fst_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 17], OperandSize::Word)
}

#[test]
fn fst_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 19], OperandSize::Dword)
}

#[test]
fn fst_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 20, 86], OperandSize::Qword)
}

#[test]
fn fst_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 213], OperandSize::Word)
}

#[test]
fn fst_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 209], OperandSize::Dword)
}

#[test]
fn fst_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 210], OperandSize::Qword)
}

