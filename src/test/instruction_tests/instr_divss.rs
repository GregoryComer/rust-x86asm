use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 254], OperandSize::Dword)
}

#[test]
fn divss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 15], OperandSize::Dword)
}

#[test]
fn divss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 227], OperandSize::Qword)
}

#[test]
fn divss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1582843947, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 132, 158, 43, 72, 88, 94], OperandSize::Qword)
}

