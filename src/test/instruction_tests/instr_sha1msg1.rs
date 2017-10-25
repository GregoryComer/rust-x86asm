use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 255], OperandSize::Dword)
}

#[test]
fn sha1msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1035710358, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 172, 201, 150, 175, 187, 61], OperandSize::Dword)
}

#[test]
fn sha1msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 239], OperandSize::Qword)
}

#[test]
fn sha1msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 60, 89], OperandSize::Qword)
}

