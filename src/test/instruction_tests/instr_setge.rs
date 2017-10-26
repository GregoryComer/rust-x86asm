use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Word)
}

#[test]
fn setge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 5], OperandSize::Word)
}

#[test]
fn setge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Dword)
}

#[test]
fn setge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 222], OperandSize::Dword)
}

#[test]
fn setge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

#[test]
fn setge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1814410618, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 85, 122, 181, 37, 108], OperandSize::Qword)
}

#[test]
fn setge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Qword)
}

#[test]
fn setge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 72], OperandSize::Qword)
}

