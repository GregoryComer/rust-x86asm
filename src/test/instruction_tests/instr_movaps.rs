use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 199], OperandSize::Dword)
}

#[test]
fn movaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1542946880, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 164, 187, 64, 128, 247, 91], OperandSize::Dword)
}

#[test]
fn movaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 250], OperandSize::Qword)
}

#[test]
fn movaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 18], OperandSize::Qword)
}

#[test]
fn movaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 244], OperandSize::Dword)
}

#[test]
fn movaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 60, 73], OperandSize::Dword)
}

#[test]
fn movaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 227], OperandSize::Qword)
}

#[test]
fn movaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1772197147, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 20, 245, 27, 149, 161, 105], OperandSize::Qword)
}

