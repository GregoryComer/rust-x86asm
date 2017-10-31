use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 255], OperandSize::Dword)
}

#[test]
fn pmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 2075711588, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 20, 149, 100, 216, 184, 123], OperandSize::Dword)
}

#[test]
fn pmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 241], OperandSize::Qword)
}

#[test]
fn pmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 232487052, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 164, 90, 140, 120, 219, 13], OperandSize::Qword)
}

