use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 227], OperandSize::Dword)
}

#[test]
fn sha256msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1150267613, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 44, 221, 221, 176, 143, 68], OperandSize::Dword)
}

#[test]
fn sha256msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 211], OperandSize::Qword)
}

#[test]
fn sha256msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 27603456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 148, 126, 0, 50, 165, 1], OperandSize::Qword)
}

