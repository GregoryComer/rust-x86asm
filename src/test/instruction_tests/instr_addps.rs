use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 239], OperandSize::Dword)
}

#[test]
fn addps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 55], OperandSize::Dword)
}

#[test]
fn addps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 245], OperandSize::Qword)
}

#[test]
fn addps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 60, 142], OperandSize::Qword)
}

