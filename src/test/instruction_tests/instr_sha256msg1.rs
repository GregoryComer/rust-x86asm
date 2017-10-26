use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 204], OperandSize::Dword)
}

#[test]
fn sha256msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 2105369004, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 164, 137, 172, 97, 125, 125], OperandSize::Dword)
}

#[test]
fn sha256msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 252], OperandSize::Qword)
}

#[test]
fn sha256msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1929981000, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 36, 117, 72, 44, 9, 115], OperandSize::Qword)
}

