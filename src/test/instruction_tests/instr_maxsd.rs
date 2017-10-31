use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 209], OperandSize::Dword)
}

#[test]
fn maxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 324647024, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 183, 112, 184, 89, 19], OperandSize::Dword)
}

#[test]
fn maxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 239], OperandSize::Qword)
}

#[test]
fn maxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 44, 241], OperandSize::Qword)
}

