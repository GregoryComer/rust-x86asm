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
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 12157, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 130, 125, 47], OperandSize::Word)
}

#[test]
fn setg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

#[test]
fn setg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectDisplaced(EDX, 1290405890, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 130, 2, 8, 234, 76], OperandSize::Dword)
}

#[test]
fn setg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 91], OperandSize::Qword)
}

#[test]
fn setg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 243], OperandSize::Qword)
}

