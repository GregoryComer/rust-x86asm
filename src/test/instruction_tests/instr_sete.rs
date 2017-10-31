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
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectDisplaced(BX, 118, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 71, 118], OperandSize::Word)
}

#[test]
fn sete_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 193], OperandSize::Dword)
}

#[test]
fn sete_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 212276941, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 132, 184, 205, 22, 167, 12], OperandSize::Dword)
}

#[test]
fn sete_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 195], OperandSize::Qword)
}

#[test]
fn sete_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4, 139], OperandSize::Qword)
}

#[test]
fn sete_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 194], OperandSize::Qword)
}

#[test]
fn sete_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 1], OperandSize::Qword)
}

