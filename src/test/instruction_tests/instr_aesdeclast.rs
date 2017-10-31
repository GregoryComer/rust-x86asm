use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 202], OperandSize::Dword)
}

#[test]
fn aesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1919230320, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 36, 133, 112, 33, 101, 114], OperandSize::Dword)
}

#[test]
fn aesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 212], OperandSize::Qword)
}

#[test]
fn aesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 52, 115], OperandSize::Qword)
}

