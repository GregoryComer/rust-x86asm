use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 235], OperandSize::Dword)
}

#[test]
fn sha256msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 60, 192], OperandSize::Dword)
}

#[test]
fn sha256msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 214], OperandSize::Qword)
}

#[test]
fn sha256msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1853580174, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 44, 245, 142, 99, 123, 110], OperandSize::Qword)
}

