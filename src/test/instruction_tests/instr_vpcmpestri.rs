use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 249, 44], OperandSize::Dword)
}

#[test]
fn vpcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 1716165916, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 170, 28, 157, 74, 102, 111], OperandSize::Dword)
}

#[test]
fn vpcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 200, 82], OperandSize::Qword)
}

#[test]
fn vpcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 539852928, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 161, 128, 128, 45, 32, 114], OperandSize::Qword)
}

