use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 214], OperandSize::Dword)
}

#[test]
fn addsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1710557320, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 154, 136, 8, 245, 101], OperandSize::Dword)
}

#[test]
fn addsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 204], OperandSize::Qword)
}

#[test]
fn addsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 795622160, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 60, 221, 16, 59, 108, 47], OperandSize::Qword)
}

