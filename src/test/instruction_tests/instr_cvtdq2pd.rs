use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 213], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 4, 154], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 236], OperandSize::Qword)
}

#[test]
fn cvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 412057405, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 20, 245, 61, 127, 143, 24], OperandSize::Qword)
}

