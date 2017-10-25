use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 200], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1527138169, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 180, 144, 121, 71, 6, 91], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 126, 24, 120, 243], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 12, 214], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 254, 24, 120, 200], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RBX, 294967615, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 120, 147, 63, 217, 148, 17], OperandSize::Qword)
}

