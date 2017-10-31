use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 195], OperandSize::Word)
}

#[test]
fn setne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 5], OperandSize::Word)
}

#[test]
fn setne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Dword)
}

#[test]
fn setne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 84138750, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 4, 205, 254, 218, 3, 5], OperandSize::Dword)
}

#[test]
fn setne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Qword)
}

#[test]
fn setne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectDisplaced(RDX, 1250563243, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 130, 171, 20, 138, 74], OperandSize::Qword)
}

#[test]
fn setne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 194], OperandSize::Qword)
}

#[test]
fn setne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledDisplaced(RCX, Four, 432145610, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 4, 141, 202, 4, 194, 25], OperandSize::Qword)
}

