use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fild_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(BX, 33, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 71, 33], OperandSize::Word)
}

#[test]
fn fild_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 2], OperandSize::Dword)
}

#[test]
fn fild_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1221097879, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 132, 91, 151, 121, 200, 72], OperandSize::Qword)
}

#[test]
fn fild_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 1], OperandSize::Word)
}

#[test]
fn fild_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 6], OperandSize::Dword)
}

#[test]
fn fild_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 4, 72], OperandSize::Qword)
}

#[test]
fn fild_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 31623, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 170, 135, 123], OperandSize::Word)
}

#[test]
fn fild_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledDisplaced(ESI, Four, 943533837, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 44, 181, 13, 47, 61, 56], OperandSize::Dword)
}

#[test]
fn fild_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 44, 130], OperandSize::Qword)
}

