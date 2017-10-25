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
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 3], OperandSize::Word)
}

#[test]
fn setge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Dword)
}

#[test]
fn setge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectDisplaced(ECX, 559074382, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 129, 78, 204, 82, 33], OperandSize::Dword)
}

#[test]
fn setge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Qword)
}

#[test]
fn setge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 7], OperandSize::Qword)
}

#[test]
fn setge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

#[test]
fn setge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1536896961, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 197, 193, 47, 155, 91], OperandSize::Qword)
}

