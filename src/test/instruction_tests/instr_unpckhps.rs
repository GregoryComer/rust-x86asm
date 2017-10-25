use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 248], OperandSize::Dword)
}

#[test]
fn unpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 579882460, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 140, 139, 220, 77, 144, 34], OperandSize::Dword)
}

#[test]
fn unpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 249], OperandSize::Qword)
}

#[test]
fn unpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1551968687, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 148, 208, 175, 41, 129, 92], OperandSize::Qword)
}

