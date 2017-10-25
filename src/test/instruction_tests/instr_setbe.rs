use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Word)
}

#[test]
fn setbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 3], OperandSize::Word)
}

#[test]
fn setbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Dword)
}

#[test]
fn setbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(EDI, 1357605590, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 135, 214, 106, 235, 80], OperandSize::Dword)
}

#[test]
fn setbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 216], OperandSize::Qword)
}

#[test]
fn setbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Qword)
}

#[test]
fn setbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(RDX, 215968491, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 130, 235, 106, 223, 12], OperandSize::Qword)
}

