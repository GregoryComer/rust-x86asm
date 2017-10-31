use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 247], OperandSize::Dword)
}

#[test]
fn vcvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 33], OperandSize::Dword)
}

#[test]
fn vcvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 219], OperandSize::Qword)
}

#[test]
fn vcvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RBX, 2038008162, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 147, 98, 137, 121, 121], OperandSize::Qword)
}

#[test]
fn vcvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 220], OperandSize::Qword)
}

#[test]
fn vcvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1535029318, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 180, 176, 70, 176, 126, 91], OperandSize::Qword)
}

#[test]
fn vcvttss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 233], OperandSize::Dword)
}

#[test]
fn vcvttss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 44, 128], OperandSize::Dword)
}

#[test]
fn vcvttss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 219], OperandSize::Qword)
}

#[test]
fn vcvttss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 20, 67], OperandSize::Qword)
}

#[test]
fn vcvttss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 254, 24, 44, 230], OperandSize::Qword)
}

#[test]
fn vcvttss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 893921609, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 188, 243, 73, 41, 72, 53], OperandSize::Qword)
}

