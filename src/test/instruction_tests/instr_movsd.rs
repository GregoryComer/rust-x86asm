use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 227], OperandSize::Dword)
}

#[test]
fn movsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 225], OperandSize::Qword)
}

#[test]
fn movsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 9], OperandSize::Dword)
}

#[test]
fn movsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1531875194, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 169, 122, 143, 78, 91], OperandSize::Qword)
}

#[test]
fn movsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 251], OperandSize::Dword)
}

#[test]
fn movsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 52, 83], OperandSize::Dword)
}

#[test]
fn movsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 246], OperandSize::Qword)
}

#[test]
fn movsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledDisplaced(RSI, Two, 533346392, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 12, 117, 88, 56, 202, 31], OperandSize::Qword)
}

