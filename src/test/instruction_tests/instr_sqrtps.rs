use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 228], OperandSize::Dword)
}

#[test]
fn sqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 47], OperandSize::Dword)
}

#[test]
fn sqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 211], OperandSize::Qword)
}

#[test]
fn sqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 35], OperandSize::Qword)
}

