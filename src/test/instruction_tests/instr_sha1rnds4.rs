use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1rnds4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 218, 65], OperandSize::Dword)
}

#[test]
fn sha1rnds4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 2051856469, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 131, 85, 216, 76, 122, 93], OperandSize::Dword)
}

#[test]
fn sha1rnds4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 247, 95], OperandSize::Qword)
}

#[test]
fn sha1rnds4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1643265507, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 164, 72, 227, 61, 242, 97, 112], OperandSize::Qword)
}

