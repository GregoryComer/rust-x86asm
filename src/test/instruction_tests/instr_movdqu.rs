use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 194], OperandSize::Dword)
}

#[test]
fn movdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 32], OperandSize::Dword)
}

#[test]
fn movdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 199], OperandSize::Qword)
}

#[test]
fn movdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1087414388, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 140, 190, 116, 160, 208, 64], OperandSize::Qword)
}

#[test]
fn movdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 200], OperandSize::Dword)
}

#[test]
fn movdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 897576055, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 164, 65, 119, 236, 127, 53], OperandSize::Dword)
}

#[test]
fn movdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 248], OperandSize::Qword)
}

#[test]
fn movdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 52, 187], OperandSize::Qword)
}

