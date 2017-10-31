use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 202], OperandSize::Dword)
}

#[test]
fn subss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 578162455, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 182, 23, 15, 118, 34], OperandSize::Dword)
}

#[test]
fn subss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 241], OperandSize::Qword)
}

#[test]
fn subss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 934351031, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 188, 64, 183, 16, 177, 55], OperandSize::Qword)
}

