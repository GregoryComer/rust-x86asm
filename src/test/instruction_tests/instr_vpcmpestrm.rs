use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 229, 60], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1389975033, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 52, 205, 249, 85, 217, 82, 114], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 252, 19], OperandSize::Qword)
}

#[test]
fn vpcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 570243821, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 4, 77, 237, 58, 253, 33, 80], OperandSize::Qword)
}

