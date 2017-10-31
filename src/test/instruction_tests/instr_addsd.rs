use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 238], OperandSize::Dword)
}

#[test]
fn addsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 63], OperandSize::Dword)
}

#[test]
fn addsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 213], OperandSize::Qword)
}

#[test]
fn addsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 1705818093, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 147, 237, 183, 172, 101], OperandSize::Qword)
}

