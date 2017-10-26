use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 233, 104], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 16, 83], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 224, 88], OperandSize::Qword)
}

#[test]
fn vpcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 60, 82, 49], OperandSize::Qword)
}

