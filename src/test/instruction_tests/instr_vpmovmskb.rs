use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 211], OperandSize::Dword)
}

#[test]
fn vpmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 234], OperandSize::Qword)
}

#[test]
fn vpmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(EBP)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 232], OperandSize::Dword)
}

#[test]
fn vpmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RBP)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 239], OperandSize::Qword)
}

