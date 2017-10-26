use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 237], OperandSize::Dword)
}

#[test]
fn aesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 265957622, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 188, 146, 246, 48, 218, 15], OperandSize::Dword)
}

#[test]
fn aesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 218], OperandSize::Qword)
}

#[test]
fn aesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RBX, 858107522, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 171, 130, 174, 37, 51], OperandSize::Qword)
}

