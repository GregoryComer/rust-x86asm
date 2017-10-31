use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 225], OperandSize::Dword)
}

#[test]
fn haddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1736047931, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 140, 127, 59, 253, 121, 103], OperandSize::Dword)
}

#[test]
fn haddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 219], OperandSize::Qword)
}

#[test]
fn haddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 33], OperandSize::Qword)
}

