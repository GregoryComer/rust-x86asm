use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 193], OperandSize::Dword)
}

#[test]
fn sha256msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 12, 71], OperandSize::Dword)
}

#[test]
fn sha256msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 246], OperandSize::Qword)
}

#[test]
fn sha256msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 60, 127], OperandSize::Qword)
}

