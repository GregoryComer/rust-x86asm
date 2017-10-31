use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Word)
}

#[test]
fn setge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 230, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 129, 230, 0], OperandSize::Word)
}

#[test]
fn setge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Dword)
}

#[test]
fn setge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1077690421, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 132, 246, 53, 64, 60, 64], OperandSize::Dword)
}

#[test]
fn setge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Qword)
}

#[test]
fn setge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 199], OperandSize::Qword)
}

#[test]
fn setge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

#[test]
fn setge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectDisplaced(RAX, 16850623, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 128, 191, 30, 1, 1], OperandSize::Qword)
}

