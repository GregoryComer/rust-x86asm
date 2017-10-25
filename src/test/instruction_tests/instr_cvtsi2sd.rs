use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 220], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 446565381, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 148, 144, 5, 12, 158, 26], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 201], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 23], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 215], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1038996243, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 180, 89, 19, 211, 237, 61], OperandSize::Qword)
}

