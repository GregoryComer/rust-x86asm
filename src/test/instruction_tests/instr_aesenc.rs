use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 204], OperandSize::Dword)
}

#[test]
fn aesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 236189873, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 147, 177, 248, 19, 14], OperandSize::Dword)
}

#[test]
fn aesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 233], OperandSize::Qword)
}

#[test]
fn aesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 51], OperandSize::Qword)
}

