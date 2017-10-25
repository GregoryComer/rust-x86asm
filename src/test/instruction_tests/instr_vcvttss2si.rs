use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 254], OperandSize::Dword)
}

#[test]
fn vcvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EBX, 1571558355, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 187, 211, 19, 172, 93], OperandSize::Dword)
}

#[test]
fn vcvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 232], OperandSize::Qword)
}

#[test]
fn vcvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RSI, 420105321, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 158, 105, 76, 10, 25], OperandSize::Qword)
}

#[test]
fn vcvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 246], OperandSize::Qword)
}

#[test]
fn vcvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 344501069, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 44, 69, 77, 171, 136, 20], OperandSize::Qword)
}

#[test]
fn vcvttss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 239], OperandSize::Dword)
}

#[test]
fn vcvttss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 44, 147], OperandSize::Dword)
}

#[test]
fn vcvttss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 126, 24, 44, 228], OperandSize::Qword)
}

#[test]
fn vcvttss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 8], OperandSize::Qword)
}

#[test]
fn vcvttss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 254, 24, 44, 218], OperandSize::Qword)
}

#[test]
fn vcvttss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RCX, 694856006, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 145, 70, 169, 106, 41], OperandSize::Qword)
}

