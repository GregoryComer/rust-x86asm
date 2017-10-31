use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 207, 41], OperandSize::Dword)
}

#[test]
fn vpcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1133095423, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 44, 125, 255, 169, 137, 67, 22], OperandSize::Dword)
}

#[test]
fn vpcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 215, 84], OperandSize::Qword)
}

#[test]
fn vpcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 33, 40], OperandSize::Qword)
}

