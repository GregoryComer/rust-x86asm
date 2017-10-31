use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 223], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EDX, 448685868, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 186, 44, 103, 190, 26], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 214], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 15], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 220], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 60, 120], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 44, 216], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDI, 327932479, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 143, 63, 218, 139, 19], OperandSize::Dword)
}

#[test]
fn vcvttsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 127, 24, 44, 232], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RCX, 1918631374, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 161, 206, 253, 91, 114], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 255, 24, 44, 255], OperandSize::Qword)
}

#[test]
fn vcvttsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 2070824032, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 180, 71, 96, 68, 110, 123], OperandSize::Qword)
}

