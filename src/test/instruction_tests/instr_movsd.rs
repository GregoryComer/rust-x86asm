use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 199], OperandSize::Dword)
}

#[test]
fn movsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 225], OperandSize::Qword)
}

#[test]
fn movsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 1654226069, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 160, 149, 124, 153, 98], OperandSize::Dword)
}

#[test]
fn movsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 136908549, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 156, 182, 5, 15, 41, 8], OperandSize::Qword)
}

#[test]
fn movsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 202], OperandSize::Dword)
}

#[test]
fn movsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectDisplaced(ESI, 1972563570, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 190, 114, 238, 146, 117], OperandSize::Dword)
}

#[test]
fn movsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 215], OperandSize::Qword)
}

#[test]
fn movsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledDisplaced(RDX, Four, 757549829, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 60, 149, 5, 75, 39, 45], OperandSize::Qword)
}

