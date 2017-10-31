use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Word)
}

#[test]
fn setl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 6219, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 130, 75, 24], OperandSize::Word)
}

#[test]
fn setl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Dword)
}

#[test]
fn setl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 1], OperandSize::Dword)
}

#[test]
fn setl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 1], OperandSize::Qword)
}

#[test]
fn setl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

#[test]
fn setl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(RDI, 1312301784, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 135, 216, 34, 56, 78], OperandSize::Qword)
}

