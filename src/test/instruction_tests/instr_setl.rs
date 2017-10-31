use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Word)
}

#[test]
fn setl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 5], OperandSize::Word)
}

#[test]
fn setl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Dword)
}

#[test]
fn setl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(EBX, 41679108, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 131, 4, 249, 123, 2], OperandSize::Dword)
}

#[test]
fn setl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 180459590, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 132, 120, 70, 152, 193, 10], OperandSize::Qword)
}

#[test]
fn setl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

#[test]
fn setl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 121], OperandSize::Qword)
}

