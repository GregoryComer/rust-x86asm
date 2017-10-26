use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 201], OperandSize::Dword)
}

#[test]
fn pmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 12, 147], OperandSize::Dword)
}

#[test]
fn pmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 255], OperandSize::Qword)
}

#[test]
fn pmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1814765692, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 36, 141, 124, 32, 43, 108], OperandSize::Qword)
}

