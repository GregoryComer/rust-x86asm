use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 248], OperandSize::Dword)
}

#[test]
fn unpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 973462898, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 185, 114, 221, 5, 58], OperandSize::Dword)
}

#[test]
fn unpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 231], OperandSize::Qword)
}

#[test]
fn unpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1216059686, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 131, 38, 153, 123, 72], OperandSize::Qword)
}

