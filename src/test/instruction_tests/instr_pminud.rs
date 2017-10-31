use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 212], OperandSize::Dword)
}

#[test]
fn pminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1067331070, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 60, 197, 254, 45, 158, 63], OperandSize::Dword)
}

#[test]
fn pminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 208], OperandSize::Qword)
}

#[test]
fn pminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 2123425895, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 147, 103, 232, 144, 126], OperandSize::Qword)
}

