use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 255], OperandSize::Dword)
}

#[test]
fn pmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 12, 250], OperandSize::Dword)
}

#[test]
fn pmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 245], OperandSize::Qword)
}

#[test]
fn pmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1962981444, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 148, 88, 68, 184, 0, 117], OperandSize::Qword)
}

