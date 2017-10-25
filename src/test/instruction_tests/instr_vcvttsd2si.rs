use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 215], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 16], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 212], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RSI, 1889214059, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 166, 107, 30, 155, 112], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 215], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 44, 208], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 44, 252], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 52, 177], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 127, 24, 44, 247], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 661181059, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 180, 88, 131, 210, 104, 39], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 255, 24, 44, 204], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 52, 87], OperandSize::Qword)
}

