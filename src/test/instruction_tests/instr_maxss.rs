use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 216], OperandSize::Dword)
}

#[test]
fn maxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 747679278, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 187, 46, 174, 144, 44], OperandSize::Dword)
}

#[test]
fn maxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 249], OperandSize::Qword)
}

#[test]
fn maxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 52, 222], OperandSize::Qword)
}

