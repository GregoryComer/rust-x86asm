use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 552631767, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 20, 149, 215, 125, 240, 32], OperandSize::Dword)
}

#[test]
fn movlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 56], OperandSize::Qword)
}

#[test]
fn movlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1232981715, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 60, 221, 211, 206, 125, 73], OperandSize::Dword)
}

#[test]
fn movlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1914992316, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 28, 93, 188, 118, 36, 114], OperandSize::Qword)
}

