use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 223, 235], OperandSize::Dword)
}

#[test]
fn vaesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 124397879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 223, 20, 69, 55, 41, 106, 7], OperandSize::Dword)
}

#[test]
fn vaesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 223, 245], OperandSize::Qword)
}

#[test]
fn vaesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 223, 44, 193], OperandSize::Qword)
}

