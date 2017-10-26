use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 234], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1642009910, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 36, 181, 54, 21, 223, 97], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 218], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1192090133, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 44, 253, 21, 218, 13, 71], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 225], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RSI, 1623798710, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 150, 182, 51, 201, 96], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 44, 202], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 1179419810, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 153, 162, 132, 76, 70], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 127, 24, 44, 206], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 32], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 255, 24, 44, 241], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RSI, 66445495, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 142, 183, 224, 245, 3], OperandSize::Qword)
}

