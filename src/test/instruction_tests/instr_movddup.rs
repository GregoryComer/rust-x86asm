use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 249], OperandSize::Dword)
}

#[test]
fn movddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 221896184, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 155, 248, 221, 57, 13], OperandSize::Dword)
}

#[test]
fn movddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 228], OperandSize::Qword)
}

#[test]
fn movddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1589174825, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 44, 245, 41, 226, 184, 94], OperandSize::Qword)
}

