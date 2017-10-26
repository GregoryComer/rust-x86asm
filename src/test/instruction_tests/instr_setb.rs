use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Word)
}

#[test]
fn setb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 178, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 131, 178, 0], OperandSize::Word)
}

#[test]
fn setb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

#[test]
fn setb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 86], OperandSize::Dword)
}

#[test]
fn setb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 3], OperandSize::Qword)
}

#[test]
fn setb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 1], OperandSize::Qword)
}

