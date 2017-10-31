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
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 2], OperandSize::Word)
}

#[test]
fn setne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Dword)
}

#[test]
fn setne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 0], OperandSize::Dword)
}

#[test]
fn setne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Qword)
}

#[test]
fn setne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectDisplaced(RDI, 1615416559, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 135, 239, 76, 73, 96], OperandSize::Qword)
}

#[test]
fn setne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 195], OperandSize::Qword)
}

#[test]
fn setne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 4, 255], OperandSize::Qword)
}

