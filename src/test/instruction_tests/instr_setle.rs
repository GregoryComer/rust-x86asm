use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Word)
}

#[test]
fn setle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 1], OperandSize::Word)
}

#[test]
fn setle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Dword)
}

#[test]
fn setle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 6], OperandSize::Dword)
}

#[test]
fn setle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Qword)
}

#[test]
fn setle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 72], OperandSize::Qword)
}

#[test]
fn setle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledDisplaced(RDX, Two, 854966802, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 85, 18, 194, 245, 50], OperandSize::Qword)
}

