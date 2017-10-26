use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 249], OperandSize::Dword)
}

#[test]
fn vcvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 55], OperandSize::Dword)
}

#[test]
fn vcvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 236], OperandSize::Qword)
}

#[test]
fn vcvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 23], OperandSize::Qword)
}

#[test]
fn vcvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 209], OperandSize::Qword)
}

#[test]
fn vcvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1596874515, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 164, 178, 19, 95, 46, 95], OperandSize::Qword)
}

#[test]
fn vcvttss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 235], OperandSize::Dword)
}

#[test]
fn vcvttss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 20, 222], OperandSize::Dword)
}

#[test]
fn vcvttss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 126, 24, 44, 233], OperandSize::Qword)
}

#[test]
fn vcvttss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 60, 131], OperandSize::Qword)
}

#[test]
fn vcvttss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 254, 24, 44, 246], OperandSize::Qword)
}

#[test]
fn vcvttss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1996617520, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 148, 64, 48, 247, 1, 119], OperandSize::Qword)
}

