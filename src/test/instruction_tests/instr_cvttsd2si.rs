use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 201], OperandSize::Dword)
}

#[test]
fn cvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 12, 130], OperandSize::Dword)
}

#[test]
fn cvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 205], OperandSize::Qword)
}

#[test]
fn cvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RDX, 808715181, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 154, 173, 3, 52, 48], OperandSize::Qword)
}

#[test]
fn cvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 246], OperandSize::Qword)
}

#[test]
fn cvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 52, 91], OperandSize::Qword)
}

