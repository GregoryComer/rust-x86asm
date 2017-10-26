use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 246], OperandSize::Dword)
}

#[test]
fn pcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1941458517, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 52, 85, 85, 78, 184, 115], OperandSize::Dword)
}

#[test]
fn pcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 199], OperandSize::Qword)
}

#[test]
fn pcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 6], OperandSize::Qword)
}

