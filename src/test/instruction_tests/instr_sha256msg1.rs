use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 234], OperandSize::Dword)
}

#[test]
fn sha256msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 7], OperandSize::Dword)
}

#[test]
fn sha256msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 202], OperandSize::Qword)
}

#[test]
fn sha256msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1154365828, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 169, 132, 57, 206, 68], OperandSize::Qword)
}

