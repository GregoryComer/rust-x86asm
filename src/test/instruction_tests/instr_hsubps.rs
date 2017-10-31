use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 199], OperandSize::Dword)
}

#[test]
fn hsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 408900125, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 140, 94, 29, 82, 95, 24], OperandSize::Dword)
}

#[test]
fn hsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 227], OperandSize::Qword)
}

#[test]
fn hsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 44, 201], OperandSize::Qword)
}

