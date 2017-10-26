use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 246], OperandSize::Dword)
}

#[test]
fn haddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 200260404, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 163, 52, 187, 239, 11], OperandSize::Dword)
}

#[test]
fn haddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 205], OperandSize::Qword)
}

#[test]
fn haddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1917288408, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 4, 85, 216, 127, 71, 114], OperandSize::Qword)
}

