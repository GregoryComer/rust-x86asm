use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 215], OperandSize::Dword)
}

#[test]
fn pmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1020145559, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 28, 77, 151, 47, 206, 60], OperandSize::Dword)
}

#[test]
fn pmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 206], OperandSize::Qword)
}

#[test]
fn pmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 765270907, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 137, 123, 27, 157, 45], OperandSize::Qword)
}

