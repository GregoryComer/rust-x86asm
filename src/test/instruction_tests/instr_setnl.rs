use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Word)
}

#[test]
fn setnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 3], OperandSize::Word)
}

#[test]
fn setnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Dword)
}

#[test]
fn setnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 23082364, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 245, 124, 53, 96, 1], OperandSize::Dword)
}

#[test]
fn setnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Qword)
}

#[test]
fn setnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectDisplaced(RCX, 1121488648, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 129, 8, 143, 216, 66], OperandSize::Qword)
}

#[test]
fn setnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Qword)
}

#[test]
fn setnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 6], OperandSize::Qword)
}

