use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 216], OperandSize::Dword)
}

#[test]
fn vcvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 35], OperandSize::Dword)
}

#[test]
fn vcvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 209], OperandSize::Qword)
}

#[test]
fn vcvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 44, 129], OperandSize::Qword)
}

#[test]
fn vcvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 213], OperandSize::Qword)
}

#[test]
fn vcvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 631731051, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 20, 253, 107, 115, 167, 37], OperandSize::Qword)
}

#[test]
fn vcvtss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 88, 45, 211], OperandSize::Dword)
}

#[test]
fn vcvtss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EAX, 620430682, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 160, 90, 5, 251, 36], OperandSize::Dword)
}

#[test]
fn vcvtss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 126, 120, 45, 246], OperandSize::Qword)
}

#[test]
fn vcvtss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 472715329, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 28, 93, 65, 16, 45, 28], OperandSize::Qword)
}

#[test]
fn vcvtss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 254, 56, 45, 236], OperandSize::Qword)
}

#[test]
fn vcvtss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 889881024, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 36, 69, 192, 129, 10, 53], OperandSize::Qword)
}

