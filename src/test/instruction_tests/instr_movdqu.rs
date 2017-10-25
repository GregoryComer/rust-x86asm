use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 220], OperandSize::Dword)
}

#[test]
fn movdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 28, 206], OperandSize::Dword)
}

#[test]
fn movdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 195], OperandSize::Qword)
}

#[test]
fn movdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 31], OperandSize::Qword)
}

#[test]
fn movdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 247], OperandSize::Dword)
}

#[test]
fn movdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 8], OperandSize::Dword)
}

#[test]
fn movdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 204], OperandSize::Qword)
}

#[test]
fn movdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 44, 191], OperandSize::Qword)
}

