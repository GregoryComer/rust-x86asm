use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 197], OperandSize::Dword)
}

#[test]
fn movdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1084463036, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 140, 223, 188, 151, 163, 64], OperandSize::Dword)
}

#[test]
fn movdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 213], OperandSize::Qword)
}

#[test]
fn movdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1981575992, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 184, 56, 115, 28, 118], OperandSize::Qword)
}

#[test]
fn movdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 238], OperandSize::Dword)
}

#[test]
fn movdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 44, 218], OperandSize::Dword)
}

#[test]
fn movdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 218], OperandSize::Qword)
}

#[test]
fn movdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 44, 202], OperandSize::Qword)
}

