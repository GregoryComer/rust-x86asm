use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 244], OperandSize::Dword)
}

#[test]
fn kmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K6)), operand2: Some(IndirectDisplaced(EDX, 1480057489, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 178, 145, 226, 55, 88], OperandSize::Dword)
}

#[test]
fn kmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 226], OperandSize::Qword)
}

#[test]
fn kmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 36, 250], OperandSize::Qword)
}

#[test]
fn kmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1597371951, Some(OperandSize::Dword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 60, 141, 47, 246, 53, 95], OperandSize::Dword)
}

#[test]
fn kmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 49], OperandSize::Qword)
}

#[test]
fn kmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 205], OperandSize::Dword)
}

#[test]
fn kmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 231], OperandSize::Qword)
}

#[test]
fn kmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 238], OperandSize::Dword)
}

#[test]
fn kmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 245], OperandSize::Qword)
}

