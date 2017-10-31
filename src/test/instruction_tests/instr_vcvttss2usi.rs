use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 227], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 167848412, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 52, 149, 220, 41, 1, 10], OperandSize::Dword)
}

#[test]
fn vcvttss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 230], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 23], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 254, 24, 120, 214], OperandSize::Qword)
}

#[test]
fn vcvttss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 120, 14], OperandSize::Qword)
}

