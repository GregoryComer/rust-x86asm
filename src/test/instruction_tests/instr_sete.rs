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
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectDisplaced(BP, 2851, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 134, 35, 11], OperandSize::Word)
}

#[test]
fn sete_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Dword)
}

#[test]
fn sete_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 1], OperandSize::Dword)
}

#[test]
fn sete_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Qword)
}

#[test]
fn sete_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 7], OperandSize::Qword)
}

#[test]
fn sete_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Qword)
}

#[test]
fn sete_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledDisplaced(RDX, Two, 590489889, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4, 85, 33, 41, 50, 35], OperandSize::Qword)
}

