use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 242, 82], OperandSize::Dword)
}

#[test]
fn vpcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 18, 11], OperandSize::Dword)
}

#[test]
fn vpcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 210, 5], OperandSize::Qword)
}

#[test]
fn vpcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 1533933200, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 177, 144, 246, 109, 91, 85], OperandSize::Qword)
}

