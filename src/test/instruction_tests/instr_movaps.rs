use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 239], OperandSize::Dword)
}

#[test]
fn movaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1386533526, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 180, 127, 150, 210, 164, 82], OperandSize::Dword)
}

#[test]
fn movaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 194], OperandSize::Qword)
}

#[test]
fn movaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1195508778, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 156, 208, 42, 4, 66, 71], OperandSize::Qword)
}

#[test]
fn movaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 217], OperandSize::Dword)
}

#[test]
fn movaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectDisplaced(EDX, 932348020, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 138, 116, 128, 146, 55], OperandSize::Dword)
}

#[test]
fn movaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 227], OperandSize::Qword)
}

#[test]
fn movaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 227593554, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 148, 193, 82, 205, 144, 13], OperandSize::Qword)
}

