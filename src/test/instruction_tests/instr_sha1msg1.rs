use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 241], OperandSize::Dword)
}

#[test]
fn sha1msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 4, 210], OperandSize::Dword)
}

#[test]
fn sha1msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 197], OperandSize::Qword)
}

#[test]
fn sha1msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1525904724, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 140, 210, 84, 117, 243, 90], OperandSize::Qword)
}

