use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 246], OperandSize::Dword)
}

#[test]
fn mulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1612534318, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 28, 77, 46, 82, 29, 96], OperandSize::Dword)
}

#[test]
fn mulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 252], OperandSize::Qword)
}

#[test]
fn mulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1856868103, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 28, 181, 7, 143, 173, 110], OperandSize::Qword)
}

