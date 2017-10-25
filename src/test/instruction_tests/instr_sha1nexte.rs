use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1nexte_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 234], OperandSize::Dword)
}

#[test]
fn sha1nexte_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1976845204, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 172, 74, 148, 67, 212, 117], OperandSize::Dword)
}

#[test]
fn sha1nexte_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 214], OperandSize::Qword)
}

#[test]
fn sha1nexte_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 66416411, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 176, 27, 111, 245, 3], OperandSize::Qword)
}

