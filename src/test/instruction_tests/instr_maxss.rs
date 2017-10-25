use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 234], OperandSize::Dword)
}

#[test]
fn maxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 51], OperandSize::Dword)
}

#[test]
fn maxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 211], OperandSize::Qword)
}

#[test]
fn maxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDX, 462172182, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 170, 22, 48, 140, 27], OperandSize::Qword)
}

