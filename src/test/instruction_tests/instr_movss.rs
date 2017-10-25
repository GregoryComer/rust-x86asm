use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 202], OperandSize::Dword)
}

#[test]
fn movss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 213], OperandSize::Qword)
}

#[test]
fn movss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 844077036, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 156, 134, 236, 151, 79, 50], OperandSize::Dword)
}

#[test]
fn movss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 60, 135], OperandSize::Qword)
}

#[test]
fn movss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 205], OperandSize::Dword)
}

#[test]
fn movss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1271196441, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 156, 143, 25, 235, 196, 75], OperandSize::Dword)
}

#[test]
fn movss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 251], OperandSize::Qword)
}

#[test]
fn movss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 60, 216], OperandSize::Qword)
}

