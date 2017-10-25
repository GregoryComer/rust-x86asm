use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdq2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQ2Q, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 214, 192], OperandSize::Word)
}

#[test]
fn movdq2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQ2Q, operand1: Some(Direct(MM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 214, 217], OperandSize::Dword)
}

#[test]
fn movdq2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQ2Q, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 214, 192], OperandSize::Qword)
}

