use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 214], OperandSize::Dword)
}

#[test]
fn pminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 305619396, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 132, 129, 196, 97, 55, 18], OperandSize::Dword)
}

#[test]
fn pminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 199], OperandSize::Qword)
}

#[test]
fn pminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 1], OperandSize::Qword)
}

