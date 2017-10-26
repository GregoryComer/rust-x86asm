use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Word)
}

#[test]
fn setc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectDisplaced(DI, 216, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 133, 216, 0], OperandSize::Word)
}

#[test]
fn setc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Dword)
}

#[test]
fn setc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 2274321, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 217, 17, 180, 34, 0], OperandSize::Dword)
}

#[test]
fn setc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1830284042, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 77, 10, 235, 23, 109], OperandSize::Qword)
}

#[test]
fn setc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 1], OperandSize::Qword)
}

