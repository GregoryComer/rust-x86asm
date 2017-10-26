use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 18, 4, 255], OperandSize::Dword)
}

#[test]
fn vmovlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 18, 60, 248], OperandSize::Qword)
}

#[test]
fn vmovlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 18013652, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 18, 179, 212, 221, 18, 1], OperandSize::Dword)
}

#[test]
fn vmovlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RDX, 1901540103, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 60, 0, 18, 162, 7, 51, 87, 113], OperandSize::Qword)
}

#[test]
fn vmovlps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 28, 143], OperandSize::Dword)
}

#[test]
fn vmovlps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 2076929116, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 164, 91, 92, 108, 203, 123], OperandSize::Qword)
}

#[test]
fn vmovlps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1428242496, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 44, 125, 64, 64, 33, 85], OperandSize::Dword)
}

#[test]
fn vmovlps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledDisplaced(RCX, Four, 828615266, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 19, 36, 141, 98, 170, 99, 49], OperandSize::Qword)
}

