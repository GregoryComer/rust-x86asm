use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 211], OperandSize::Dword)
}

#[test]
fn pminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1612507486, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 20, 93, 94, 233, 28, 96], OperandSize::Dword)
}

#[test]
fn pminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 218], OperandSize::Qword)
}

#[test]
fn pminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 46], OperandSize::Qword)
}

