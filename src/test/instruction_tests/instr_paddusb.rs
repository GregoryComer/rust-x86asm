use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 224], OperandSize::Dword)
}

#[test]
fn paddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 40], OperandSize::Dword)
}

#[test]
fn paddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 226], OperandSize::Qword)
}

#[test]
fn paddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 10], OperandSize::Qword)
}

#[test]
fn paddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 204], OperandSize::Dword)
}

#[test]
fn paddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 605471221, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 187, 245, 193, 22, 36], OperandSize::Dword)
}

#[test]
fn paddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 204], OperandSize::Qword)
}

#[test]
fn paddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 1121598604, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 187, 140, 60, 218, 66], OperandSize::Qword)
}

