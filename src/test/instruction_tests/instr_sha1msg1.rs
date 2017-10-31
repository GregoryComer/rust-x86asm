use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 195], OperandSize::Dword)
}

#[test]
fn sha1msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1232471177, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 148, 130, 137, 4, 118, 73], OperandSize::Dword)
}

#[test]
fn sha1msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 226], OperandSize::Qword)
}

#[test]
fn sha1msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 1805589, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 128, 21, 141, 27, 0], OperandSize::Qword)
}

