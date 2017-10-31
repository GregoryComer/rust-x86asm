use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 250], OperandSize::Dword)
}

#[test]
fn pblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1552908785, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 140, 79, 241, 129, 143, 92], OperandSize::Dword)
}

#[test]
fn pblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 201], OperandSize::Qword)
}

#[test]
fn pblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 4, 112], OperandSize::Qword)
}

