use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 247], OperandSize::Dword)
}

#[test]
fn sqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 4, 184], OperandSize::Dword)
}

#[test]
fn sqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 225], OperandSize::Qword)
}

#[test]
fn sqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 44, 138], OperandSize::Qword)
}

