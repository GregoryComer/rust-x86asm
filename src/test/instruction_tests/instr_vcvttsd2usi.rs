use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 211], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 934354325, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 180, 199, 149, 29, 177, 55], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 127, 24, 120, 232], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 921327477, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 140, 81, 117, 87, 234, 54], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 255, 24, 120, 254], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RCX, 1234346776, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 120, 185, 24, 163, 146, 73], OperandSize::Qword)
}

