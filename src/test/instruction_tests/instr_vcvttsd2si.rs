use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 245], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 12, 91], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 236], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 46], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 201], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RSI, 114240213, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 150, 213, 42, 207, 6], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 44, 203], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 851973293, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 148, 254, 173, 20, 200, 50], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 127, 24, 44, 215], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1025676534, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 148, 113, 246, 148, 34, 61], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 255, 24, 44, 208], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 248866928, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 140, 119, 112, 104, 213, 14], OperandSize::Qword)
}

