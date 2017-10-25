use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Word)
}

#[test]
fn setna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 12871, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 130, 71, 50], OperandSize::Word)
}

#[test]
fn setna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Dword)
}

#[test]
fn setna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 178048008, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 132, 70, 8, 204, 156, 10], OperandSize::Dword)
}

#[test]
fn setna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

#[test]
fn setna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectDisplaced(RSI, 1162562057, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 134, 9, 74, 75, 69], OperandSize::Qword)
}

#[test]
fn setna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Qword)
}

#[test]
fn setna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 140590342, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 132, 183, 6, 61, 97, 8], OperandSize::Qword)
}

