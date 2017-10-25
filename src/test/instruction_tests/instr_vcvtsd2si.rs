use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 251], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1746386837, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 148, 217, 149, 191, 23, 104], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 252], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 2080901948, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 44, 213, 60, 11, 8, 124], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 252], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 54], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 56, 45, 225], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1197718927, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 172, 113, 143, 189, 99, 71], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 127, 24, 45, 228], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 52, 115], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 255, 120, 45, 246], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 57], OperandSize::Qword)
}

