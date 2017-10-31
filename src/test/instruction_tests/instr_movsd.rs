use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 211], OperandSize::Dword)
}

#[test]
fn movsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 251], OperandSize::Qword)
}

#[test]
fn movsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 428619298, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 166, 34, 54, 140, 25], OperandSize::Dword)
}

#[test]
fn movsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1688312080, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 164, 202, 16, 153, 161, 100], OperandSize::Qword)
}

#[test]
fn movsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 229], OperandSize::Dword)
}

#[test]
fn movsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 44, 126], OperandSize::Dword)
}

#[test]
fn movsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 229], OperandSize::Qword)
}

#[test]
fn movsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 834160516, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 52, 189, 132, 71, 184, 49], OperandSize::Qword)
}

