use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 246], OperandSize::Dword)
}

#[test]
fn sha256msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 719354670, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 134, 46, 123, 224, 42], OperandSize::Dword)
}

#[test]
fn sha256msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 224], OperandSize::Qword)
}

#[test]
fn sha256msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 621447382, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 12, 133, 214, 136, 10, 37], OperandSize::Qword)
}

