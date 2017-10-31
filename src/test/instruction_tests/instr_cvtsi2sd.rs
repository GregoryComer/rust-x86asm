use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 205], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 52, 209], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 218], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 1], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 211], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 47], OperandSize::Qword)
}

