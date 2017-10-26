use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 248], OperandSize::Dword)
}

#[test]
fn cvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1025923251, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 36, 77, 179, 88, 38, 61], OperandSize::Dword)
}

#[test]
fn cvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 230], OperandSize::Qword)
}

#[test]
fn cvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 311215908, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 44, 125, 36, 199, 140, 18], OperandSize::Qword)
}

