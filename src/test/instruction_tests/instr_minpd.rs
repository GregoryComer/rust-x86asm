use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 226], OperandSize::Dword)
}

#[test]
fn minpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 253184767, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 132, 74, 255, 74, 23, 15], OperandSize::Dword)
}

#[test]
fn minpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 242], OperandSize::Qword)
}

#[test]
fn minpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 404635255, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 180, 203, 119, 62, 30, 24], OperandSize::Qword)
}

