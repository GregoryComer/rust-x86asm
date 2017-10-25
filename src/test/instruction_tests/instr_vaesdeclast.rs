use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 223, 224], OperandSize::Dword)
}

#[test]
fn vaesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1801891816, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 223, 168, 232, 175, 102, 107], OperandSize::Dword)
}

#[test]
fn vaesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 223, 248], OperandSize::Qword)
}

#[test]
fn vaesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 949357430, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 223, 52, 197, 118, 11, 150, 56], OperandSize::Qword)
}

