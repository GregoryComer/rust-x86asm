use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1rnds4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 227, 121], OperandSize::Dword)
}

#[test]
fn sha1rnds4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 63, 96], OperandSize::Dword)
}

#[test]
fn sha1rnds4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 193, 83], OperandSize::Qword)
}

#[test]
fn sha1rnds4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 900371460, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 158, 4, 148, 170, 53, 115], OperandSize::Qword)
}

