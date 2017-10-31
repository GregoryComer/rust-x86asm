use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 255], OperandSize::Dword)
}

#[test]
fn addps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EAX, 92506026, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 152, 170, 135, 131, 5], OperandSize::Dword)
}

#[test]
fn addps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 251], OperandSize::Qword)
}

#[test]
fn addps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 775623628, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 20, 157, 204, 19, 59, 46], OperandSize::Qword)
}

