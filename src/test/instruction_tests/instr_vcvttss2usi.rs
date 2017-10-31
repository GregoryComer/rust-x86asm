use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 202], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 58], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 126, 24, 120, 252], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 36, 144], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 254, 24, 120, 210], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1543963392, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 120, 156, 87, 0, 3, 7, 92], OperandSize::Qword)
}

