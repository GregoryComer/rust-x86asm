use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 218, 64], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 44, 248, 68], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 232, 2], OperandSize::Qword)
}

#[test]
fn vpcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 38, 118], OperandSize::Qword)
}

