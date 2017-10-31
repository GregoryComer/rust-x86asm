use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Word)
}

#[test]
fn setb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectDisplaced(SI, 9087, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 127, 35], OperandSize::Word)
}

#[test]
fn setb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

#[test]
fn setb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 2], OperandSize::Dword)
}

#[test]
fn setb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectDisplaced(RDI, 854708002, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 135, 34, 207, 241, 50], OperandSize::Qword)
}

#[test]
fn setb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1157456417, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 189, 33, 98, 253, 68], OperandSize::Qword)
}

