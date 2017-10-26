use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 208], OperandSize::Dword)
}

#[test]
fn movddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 1], OperandSize::Dword)
}

#[test]
fn movddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 204], OperandSize::Qword)
}

#[test]
fn movddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 9], OperandSize::Qword)
}

