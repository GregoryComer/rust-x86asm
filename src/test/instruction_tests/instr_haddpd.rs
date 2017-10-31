use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 225], OperandSize::Dword)
}

#[test]
fn haddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1445109071, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 148, 94, 79, 157, 34, 86], OperandSize::Dword)
}

#[test]
fn haddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 227], OperandSize::Qword)
}

#[test]
fn haddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 820785443, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 164, 218, 35, 49, 236, 48], OperandSize::Qword)
}

