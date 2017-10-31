use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 212], OperandSize::Dword)
}

#[test]
fn rsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1442137549, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 4, 213, 205, 69, 245, 85], OperandSize::Dword)
}

#[test]
fn rsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 222], OperandSize::Qword)
}

#[test]
fn rsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 692085191, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 183, 199, 97, 64, 41], OperandSize::Qword)
}

