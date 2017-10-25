use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 210, 125], OperandSize::Dword)
}

#[test]
fn vpcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 28, 209, 81], OperandSize::Dword)
}

#[test]
fn vpcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 195, 45], OperandSize::Qword)
}

#[test]
fn vpcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1604885890, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 132, 129, 130, 157, 168, 95, 60], OperandSize::Qword)
}

