use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 193], OperandSize::Word)
}

#[test]
fn setng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 22037, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 128, 21, 86], OperandSize::Word)
}

#[test]
fn setng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Dword)
}

#[test]
fn setng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 142], OperandSize::Dword)
}

#[test]
fn setng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 107872670, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 200, 158, 1, 110, 6], OperandSize::Qword)
}

#[test]
fn setng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectDisplaced(RCX, 884823158, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 129, 118, 84, 189, 52], OperandSize::Qword)
}

