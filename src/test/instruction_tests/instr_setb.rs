use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Word)
}

#[test]
fn setb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectDisplaced(BP, 73, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 70, 73], OperandSize::Word)
}

#[test]
fn setb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Dword)
}

#[test]
fn setb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 221698375, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 131, 71, 217, 54, 13], OperandSize::Dword)
}

#[test]
fn setb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 202], OperandSize::Qword)
}

#[test]
fn setb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 159], OperandSize::Qword)
}

