use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sete_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Word)
}

#[test]
fn sete_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Memory(11418, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 6, 154, 44], OperandSize::Word)
}

#[test]
fn sete_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Dword)
}

#[test]
fn sete_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectDisplaced(EDI, 1239280881, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 135, 241, 236, 221, 73], OperandSize::Dword)
}

#[test]
fn sete_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 193], OperandSize::Qword)
}

#[test]
fn sete_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 78863053, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 132, 129, 205, 90, 179, 4], OperandSize::Qword)
}

#[test]
fn sete_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Qword)
}

#[test]
fn sete_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1927607509, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4, 149, 213, 244, 228, 114], OperandSize::Qword)
}

