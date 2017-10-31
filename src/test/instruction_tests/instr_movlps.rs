use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 101064191, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 4, 149, 255, 29, 6, 6], OperandSize::Dword)
}

#[test]
fn movlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1612604995, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 44, 69, 67, 102, 30, 96], OperandSize::Qword)
}

#[test]
fn movlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 20, 91], OperandSize::Dword)
}

#[test]
fn movlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 2111930632, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 12, 213, 8, 129, 225, 125], OperandSize::Qword)
}

