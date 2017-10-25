use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Word)
}

#[test]
fn setne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Memory(1229, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 6, 205, 4], OperandSize::Word)
}

#[test]
fn setne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Dword)
}

#[test]
fn setne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1047721254, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 132, 214, 38, 245, 114, 62], OperandSize::Dword)
}

#[test]
fn setne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Qword)
}

#[test]
fn setne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectDisplaced(RSI, 954478532, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 134, 196, 47, 228, 56], OperandSize::Qword)
}

#[test]
fn setne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Qword)
}

#[test]
fn setne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 2], OperandSize::Qword)
}

