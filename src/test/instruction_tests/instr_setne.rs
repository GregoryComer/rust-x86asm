use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Word)
}

#[test]
fn setne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 20533, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 128, 53, 80], OperandSize::Word)
}

#[test]
fn setne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Dword)
}

#[test]
fn setne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectDisplaced(EBX, 639953255, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 131, 103, 233, 36, 38], OperandSize::Dword)
}

#[test]
fn setne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 195], OperandSize::Qword)
}

#[test]
fn setne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 0], OperandSize::Qword)
}

#[test]
fn setne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Qword)
}

#[test]
fn setne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 4, 80], OperandSize::Qword)
}

