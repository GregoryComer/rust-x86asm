use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1rnds4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 219, 15], OperandSize::Dword)
}

#[test]
fn sha1rnds4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 55, 14], OperandSize::Dword)
}

#[test]
fn sha1rnds4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 223, 98], OperandSize::Qword)
}

#[test]
fn sha1rnds4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 1903190799, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 175, 15, 99, 112, 113, 92], OperandSize::Qword)
}

