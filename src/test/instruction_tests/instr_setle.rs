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
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 11999, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 128, 223, 46], OperandSize::Word)
}

#[test]
fn setle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 193], OperandSize::Dword)
}

#[test]
fn setle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 3], OperandSize::Dword)
}

#[test]
fn setle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 572160051, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 114, 51, 120, 26, 34], OperandSize::Qword)
}

#[test]
fn setle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1587185486, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 77, 78, 135, 154, 94], OperandSize::Qword)
}

