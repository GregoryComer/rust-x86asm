use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 208], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 604479730, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 180, 159, 242, 160, 7, 36], OperandSize::Dword)
}

#[test]
fn vcvttsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 127, 24, 120, 235], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 36, 241], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 255, 24, 120, 243], OperandSize::Qword)
}

#[test]
fn vcvttsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 120, 52, 155], OperandSize::Qword)
}

