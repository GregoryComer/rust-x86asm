use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 88, 121, 227], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EAX, 147326640, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 168, 176, 6, 200, 8], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 127, 120, 121, 245], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RCX, 585549064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 145, 8, 197, 230, 34], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 56, 121, 212], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RSI, 361938759, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 121, 182, 71, 191, 146, 21], OperandSize::Qword)
}

