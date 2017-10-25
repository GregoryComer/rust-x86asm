use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 198], OperandSize::Dword)
}

#[test]
fn psubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 64289312, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 132, 218, 32, 250, 212, 3], OperandSize::Dword)
}

#[test]
fn psubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 206], OperandSize::Qword)
}

#[test]
fn psubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 2476331, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 172, 152, 43, 201, 37, 0], OperandSize::Qword)
}

#[test]
fn psubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 251], OperandSize::Dword)
}

#[test]
fn psubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 20, 218], OperandSize::Dword)
}

#[test]
fn psubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 198], OperandSize::Qword)
}

#[test]
fn psubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1050143387, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 140, 155, 155, 234, 151, 62], OperandSize::Qword)
}

