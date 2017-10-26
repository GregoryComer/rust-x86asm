use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 223], OperandSize::Dword)
}

#[test]
fn sha1msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 2005217077, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 178, 53, 47, 133, 119], OperandSize::Dword)
}

#[test]
fn sha1msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 203], OperandSize::Qword)
}

#[test]
fn sha1msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 1492797591, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 146, 151, 72, 250, 88], OperandSize::Qword)
}

