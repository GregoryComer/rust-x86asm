use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 223], OperandSize::Dword)
}

#[test]
fn sha256msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 12, 255], OperandSize::Dword)
}

#[test]
fn sha256msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 228], OperandSize::Qword)
}

#[test]
fn sha256msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 118911450, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 20, 253, 218, 113, 22, 7], OperandSize::Qword)
}

