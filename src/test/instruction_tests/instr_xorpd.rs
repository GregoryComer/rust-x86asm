use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 255], OperandSize::Dword)
}

#[test]
fn xorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1547782538, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 140, 130, 138, 73, 65, 92], OperandSize::Dword)
}

#[test]
fn xorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 210], OperandSize::Qword)
}

#[test]
fn xorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 545199737, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 132, 241, 121, 22, 127, 32], OperandSize::Qword)
}

