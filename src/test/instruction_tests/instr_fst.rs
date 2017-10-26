use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fst_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 585, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 147, 73, 2], OperandSize::Word)
}

#[test]
fn fst_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(EDI, Two, 163486800, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 125, 80, 156, 190, 9], OperandSize::Dword)
}

#[test]
fn fst_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1750072431, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 125, 111, 252, 79, 104], OperandSize::Qword)
}

#[test]
fn fst_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(BX, 37, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 87, 37], OperandSize::Word)
}

#[test]
fn fst_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(EBX, 571133624, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 147, 184, 206, 10, 34], OperandSize::Dword)
}

#[test]
fn fst_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1293180970, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 20, 133, 42, 96, 20, 77], OperandSize::Qword)
}

#[test]
fn fst_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 212], OperandSize::Word)
}

#[test]
fn fst_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 214], OperandSize::Dword)
}

#[test]
fn fst_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 213], OperandSize::Qword)
}

