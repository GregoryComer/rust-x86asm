use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 211], OperandSize::Dword)
}

#[test]
fn cvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 12, 83], OperandSize::Dword)
}

#[test]
fn cvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 218], OperandSize::Qword)
}

#[test]
fn cvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 52, 142], OperandSize::Qword)
}

#[test]
fn cvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 233], OperandSize::Qword)
}

#[test]
fn cvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 60, 201], OperandSize::Qword)
}

