use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 223], OperandSize::Dword)
}

#[test]
fn pmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 4, 83], OperandSize::Dword)
}

#[test]
fn pmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 198], OperandSize::Qword)
}

#[test]
fn pmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 52, 158], OperandSize::Qword)
}

