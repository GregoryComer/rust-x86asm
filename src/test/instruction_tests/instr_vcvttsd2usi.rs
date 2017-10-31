use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 202], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 559417094, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 44, 69, 6, 7, 88, 33], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 127, 24, 120, 243], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1566088195, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 164, 247, 3, 156, 88, 93], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 255, 24, 120, 215], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 1114748995, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 120, 164, 249, 67, 184, 113, 66], OperandSize::Qword)
}

