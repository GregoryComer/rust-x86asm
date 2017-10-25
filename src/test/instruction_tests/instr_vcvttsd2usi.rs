use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 204], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1849278680, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 52, 77, 216, 192, 57, 110], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 210], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1433297268, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 172, 121, 116, 97, 110, 85], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 255, 24, 120, 251], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 592190089, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 120, 28, 157, 137, 26, 76, 35], OperandSize::Qword)
}

