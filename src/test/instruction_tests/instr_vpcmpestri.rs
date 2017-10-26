use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 192, 69], OperandSize::Dword)
}

#[test]
fn vpcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 961676738, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 139, 194, 5, 82, 57, 47], OperandSize::Dword)
}

#[test]
fn vpcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 201, 18], OperandSize::Qword)
}

#[test]
fn vpcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 28, 210, 30], OperandSize::Qword)
}

