use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Word)
}

#[test]
fn setl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 4, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 64, 4], OperandSize::Word)
}

#[test]
fn setl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Dword)
}

#[test]
fn setl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(EDI, 672765856, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 135, 160, 151, 25, 40], OperandSize::Dword)
}

#[test]
fn setl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

#[test]
fn setl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1790263556, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 141, 4, 65, 181, 106], OperandSize::Qword)
}

#[test]
fn setl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1698113246, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 197, 222, 38, 55, 101], OperandSize::Qword)
}

