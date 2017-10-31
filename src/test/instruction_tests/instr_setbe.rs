use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Word)
}

#[test]
fn setbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(DI, 64, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 69, 64], OperandSize::Word)
}

#[test]
fn setbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Dword)
}

#[test]
fn setbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1006504807, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 189, 103, 11, 254, 59], OperandSize::Dword)
}

#[test]
fn setbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

#[test]
fn setbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1417858723, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 221, 163, 206, 130, 84], OperandSize::Qword)
}

#[test]
fn setbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 248], OperandSize::Qword)
}

