use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 22, 6], OperandSize::Dword)
}

#[test]
fn vmovhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 73204499, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 22, 172, 114, 19, 3, 93, 4], OperandSize::Qword)
}

#[test]
fn vmovhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 135867477, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 22, 156, 187, 85, 44, 25, 8], OperandSize::Dword)
}

#[test]
fn vmovhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1135502885, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 22, 28, 205, 37, 102, 174, 67], OperandSize::Qword)
}

#[test]
fn vmovhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 18], OperandSize::Dword)
}

#[test]
fn vmovhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 44684815, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 148, 79, 15, 214, 169, 2], OperandSize::Qword)
}

#[test]
fn vmovhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 32], OperandSize::Dword)
}

#[test]
fn vmovhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1900545335, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 23, 60, 157, 55, 5, 72, 113], OperandSize::Qword)
}

