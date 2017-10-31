use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Word)
}

#[test]
fn setle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 15, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 64, 15], OperandSize::Word)
}

#[test]
fn setle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Dword)
}

#[test]
fn setle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectDisplaced(EAX, 1645720664, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 128, 88, 180, 23, 98], OperandSize::Dword)
}

#[test]
fn setle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Qword)
}

#[test]
fn setle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1197563129, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 215, 249, 92, 97, 71], OperandSize::Qword)
}

#[test]
fn setle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Qword)
}

#[test]
fn setle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 2013868858, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 179, 58, 51, 9, 120], OperandSize::Qword)
}

