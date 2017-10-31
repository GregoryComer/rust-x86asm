use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1nexte_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 253], OperandSize::Dword)
}

#[test]
fn sha1nexte_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 25], OperandSize::Dword)
}

#[test]
fn sha1nexte_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 242], OperandSize::Qword)
}

#[test]
fn sha1nexte_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 1684601864, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 168, 8, 252, 104, 100], OperandSize::Qword)
}

