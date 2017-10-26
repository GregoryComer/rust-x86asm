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
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 2113623884, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 175, 76, 87, 251, 125], OperandSize::Dword)
}

#[test]
fn sha1nexte_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 225], OperandSize::Qword)
}

#[test]
fn sha1nexte_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 36, 246], OperandSize::Qword)
}

