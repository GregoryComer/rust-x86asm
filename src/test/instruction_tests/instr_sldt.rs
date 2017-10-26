use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 195], OperandSize::Word)
}

#[test]
fn sldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 16116, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 130, 244, 62], OperandSize::Word)
}

#[test]
fn sldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 197], OperandSize::Dword)
}

#[test]
fn sldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1412100810, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 132, 79, 202, 242, 42, 84], OperandSize::Dword)
}

#[test]
fn sldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 199], OperandSize::Qword)
}

#[test]
fn sldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledDisplaced(RAX, Two, 806377897, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 4, 69, 169, 89, 16, 48], OperandSize::Qword)
}

#[test]
fn sldt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 0, 193], OperandSize::Qword)
}

#[test]
fn sldt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1733100904, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 4, 245, 104, 5, 77, 103], OperandSize::Qword)
}

