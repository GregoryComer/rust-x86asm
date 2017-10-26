use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 245, 113], OperandSize::Dword)
}

#[test]
fn vpcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 535610345, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 52, 117, 233, 195, 236, 31, 32], OperandSize::Dword)
}

#[test]
fn vpcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 202, 40], OperandSize::Qword)
}

#[test]
fn vpcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 44, 246, 35], OperandSize::Qword)
}

