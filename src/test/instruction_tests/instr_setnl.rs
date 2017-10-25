use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Word)
}

#[test]
fn setnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectDisplaced(BX, 11270, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 135, 6, 44], OperandSize::Word)
}

#[test]
fn setnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Dword)
}

#[test]
fn setnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 2070253922, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 132, 208, 98, 145, 101, 123], OperandSize::Dword)
}

#[test]
fn setnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Qword)
}

#[test]
fn setnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 73], OperandSize::Qword)
}

#[test]
fn setnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Qword)
}

#[test]
fn setnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 339763656, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 132, 216, 200, 97, 64, 20], OperandSize::Qword)
}

