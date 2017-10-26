use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1rnds4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 192, 12], OperandSize::Dword)
}

#[test]
fn sha1rnds4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 43, 30], OperandSize::Dword)
}

#[test]
fn sha1rnds4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 252, 115], OperandSize::Qword)
}

#[test]
fn sha1rnds4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 10, 47], OperandSize::Qword)
}

