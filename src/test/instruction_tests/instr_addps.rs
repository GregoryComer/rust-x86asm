use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 199], OperandSize::Dword)
}

#[test]
fn addps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 237775278, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 179, 174, 41, 44, 14], OperandSize::Dword)
}

#[test]
fn addps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 253], OperandSize::Qword)
}

#[test]
fn addps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 58], OperandSize::Qword)
}

