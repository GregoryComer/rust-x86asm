use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 211], OperandSize::Dword)
}

#[test]
fn subpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 148890593, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 179, 225, 227, 223, 8], OperandSize::Dword)
}

#[test]
fn subpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 251], OperandSize::Qword)
}

#[test]
fn subpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1827973383, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 60, 93, 7, 169, 244, 108], OperandSize::Qword)
}

