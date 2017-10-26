use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 242], OperandSize::Dword)
}

#[test]
fn sqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 3465586, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 4, 221, 114, 225, 52, 0], OperandSize::Dword)
}

#[test]
fn sqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 253], OperandSize::Qword)
}

#[test]
fn sqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 41], OperandSize::Qword)
}

