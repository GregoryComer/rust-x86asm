use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Word)
}

#[test]
fn setg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectDisplaced(BX, 150, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 135, 150, 0], OperandSize::Word)
}

#[test]
fn setg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Dword)
}

#[test]
fn setg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(ECX, Two, 63796253, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 77, 29, 116, 205, 3], OperandSize::Dword)
}

#[test]
fn setg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Qword)
}

#[test]
fn setg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(RDX, Four, 968940052, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 149, 20, 218, 192, 57], OperandSize::Qword)
}

#[test]
fn setg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Qword)
}

#[test]
fn setg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 892762010, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 253, 154, 119, 54, 53], OperandSize::Qword)
}

