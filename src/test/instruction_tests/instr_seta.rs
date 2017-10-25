use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn seta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Word)
}

#[test]
fn seta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 2], OperandSize::Word)
}

#[test]
fn seta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Dword)
}

#[test]
fn seta_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 194631135, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 154, 223, 213, 153, 11], OperandSize::Dword)
}

#[test]
fn seta_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Qword)
}

#[test]
fn seta_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 49739777, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 145, 1, 248, 246, 2], OperandSize::Qword)
}

#[test]
fn seta_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Qword)
}

#[test]
fn seta_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RAX, Two, 657792866, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 69, 98, 31, 53, 39], OperandSize::Qword)
}

