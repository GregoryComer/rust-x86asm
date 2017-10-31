use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 250], OperandSize::Dword)
}

#[test]
fn vucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 44, 210], OperandSize::Dword)
}

#[test]
fn vucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 225], OperandSize::Qword)
}

#[test]
fn vucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 835322529, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 140, 79, 161, 2, 202, 49], OperandSize::Qword)
}

#[test]
fn vucomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 46, 245], OperandSize::Dword)
}

#[test]
fn vucomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 822360064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 156, 83, 0, 56, 4, 49], OperandSize::Dword)
}

#[test]
fn vucomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 97, 253, 24, 46, 252], OperandSize::Qword)
}

#[test]
fn vucomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RDX, 1598246259, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 46, 138, 115, 77, 67, 95], OperandSize::Qword)
}

