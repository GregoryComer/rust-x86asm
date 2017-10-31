use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Word)
}

#[test]
fn setnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 3], OperandSize::Word)
}

#[test]
fn setnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Dword)
}

#[test]
fn setnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 6], OperandSize::Dword)
}

#[test]
fn setnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

#[test]
fn setnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 592404076, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 197, 108, 94, 79, 35], OperandSize::Qword)
}

#[test]
fn setnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Qword)
}

#[test]
fn setnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 1], OperandSize::Qword)
}

