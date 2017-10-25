use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 244, 32], OperandSize::Dword)
}

#[test]
fn cmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 647895218, Some(OperandSize::Qword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 180, 90, 178, 24, 158, 38, 111], OperandSize::Dword)
}

#[test]
fn cmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 230, 123], OperandSize::Qword)
}

#[test]
fn cmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 1673222358, Some(OperandSize::Qword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 144, 214, 88, 187, 99, 78], OperandSize::Qword)
}

