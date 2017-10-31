use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 226, 70], OperandSize::Dword)
}

#[test]
fn vroundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 760216137, Some(OperandSize::Dword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 132, 120, 73, 250, 79, 45, 125], OperandSize::Dword)
}

#[test]
fn vroundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 196, 37], OperandSize::Qword)
}

#[test]
fn vroundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 401521415, Some(OperandSize::Dword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 10, 52, 221, 7, 187, 238, 23, 108], OperandSize::Qword)
}

