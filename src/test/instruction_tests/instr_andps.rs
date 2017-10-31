use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 253], OperandSize::Dword)
}

#[test]
fn andps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 853381447, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 135, 71, 145, 221, 50], OperandSize::Dword)
}

#[test]
fn andps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 253], OperandSize::Qword)
}

#[test]
fn andps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 1317190663, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 187, 7, 188, 130, 78], OperandSize::Qword)
}

