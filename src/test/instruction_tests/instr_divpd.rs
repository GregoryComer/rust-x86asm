use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 209], OperandSize::Dword)
}

#[test]
fn divpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 6], OperandSize::Dword)
}

#[test]
fn divpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 246], OperandSize::Qword)
}

#[test]
fn divpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 640327808, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 152, 128, 160, 42, 38], OperandSize::Qword)
}

