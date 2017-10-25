use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 121, 242], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1089543594, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 140, 243, 170, 29, 241, 64], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 126, 120, 121, 235], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 38], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 88, 121, 211], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1125639467, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 121, 156, 123, 43, 229, 23, 67], OperandSize::Qword)
}

