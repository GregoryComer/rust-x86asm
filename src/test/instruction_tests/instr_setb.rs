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
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Memory(8866, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 6, 162, 34], OperandSize::Word)
}

#[test]
fn setb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Dword)
}

#[test]
fn setb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1700387730, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 186, 146, 219, 89, 101], OperandSize::Dword)
}

#[test]
fn setb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1470235488, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 65, 96, 3, 162, 87], OperandSize::Qword)
}

#[test]
fn setb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1051587600, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 155, 16, 244, 173, 62], OperandSize::Qword)
}

