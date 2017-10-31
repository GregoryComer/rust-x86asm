use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 214], OperandSize::Dword)
}

#[test]
fn kmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 40], OperandSize::Dword)
}

#[test]
fn kmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 229], OperandSize::Qword)
}

#[test]
fn kmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1361768525, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 36, 69, 77, 240, 42, 81], OperandSize::Qword)
}

#[test]
fn kmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectDisplaced(EBX, 916846735, Some(OperandSize::Qword), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 179, 143, 248, 165, 54], OperandSize::Dword)
}

#[test]
fn kmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 52, 136], OperandSize::Qword)
}

#[test]
fn kmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K1)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 146, 207], OperandSize::Qword)
}

#[test]
fn kmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(RDX)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 147, 209], OperandSize::Qword)
}

