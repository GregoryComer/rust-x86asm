use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 56, 121, 200], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 347688741, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 156, 131, 37, 79, 185, 20], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 120, 121, 202], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RDI, 1161829477, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 191, 101, 28, 64, 69], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 255, 120, 121, 204], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 541645996, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 121, 12, 141, 172, 220, 72, 32], OperandSize::Qword)
}

