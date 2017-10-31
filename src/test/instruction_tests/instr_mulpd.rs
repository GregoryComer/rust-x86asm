use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 242], OperandSize::Dword)
}

#[test]
fn mulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1834563034, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 132, 155, 218, 53, 89, 109], OperandSize::Dword)
}

#[test]
fn mulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 202], OperandSize::Qword)
}

#[test]
fn mulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 52, 94], OperandSize::Qword)
}

