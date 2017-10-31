use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 121, 209], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 11], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 126, 120, 121, 206], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RBX, 1126624829, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 187, 61, 238, 38, 67], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 254, 120, 121, 240], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 121, 52, 114], OperandSize::Qword)
}

