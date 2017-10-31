use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 255], OperandSize::Dword)
}

#[test]
fn divpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1634849259, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 36, 197, 235, 209, 113, 97], OperandSize::Dword)
}

#[test]
fn divpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 226], OperandSize::Qword)
}

#[test]
fn divpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 8], OperandSize::Qword)
}

