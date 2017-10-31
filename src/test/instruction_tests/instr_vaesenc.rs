use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 220, 241], OperandSize::Dword)
}

#[test]
fn vaesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 220, 52, 248], OperandSize::Dword)
}

#[test]
fn vaesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 220, 210], OperandSize::Qword)
}

#[test]
fn vaesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 220, 24], OperandSize::Qword)
}

