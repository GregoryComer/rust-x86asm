use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 6111, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 170, 223, 23], OperandSize::Word)
}

#[test]
fn fisubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 723869918, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 78, 222, 96, 37, 43], OperandSize::Dword)
}

#[test]
fn fisubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1003105823, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 67, 31, 46, 202, 59], OperandSize::Qword)
}

#[test]
fn fisubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 24902, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 169, 70, 97], OperandSize::Word)
}

#[test]
fn fisubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 44, 211], OperandSize::Dword)
}

#[test]
fn fisubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 47], OperandSize::Qword)
}

