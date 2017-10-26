use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 241], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EDX, 671087479, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 170, 119, 251, 255, 39], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 126, 24, 120, 236], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 923100968, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 140, 147, 40, 103, 5, 55], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 254, 24, 120, 236], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 120, 55], OperandSize::Qword)
}

