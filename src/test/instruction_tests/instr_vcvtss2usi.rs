use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 88, 121, 209], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 60, 217], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 121, 225], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 735368914, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 188, 207, 210, 214, 212, 43], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 120, 121, 235], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 121, 16], OperandSize::Qword)
}

