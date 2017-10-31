use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 212], OperandSize::Dword)
}

#[test]
fn movupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 25], OperandSize::Dword)
}

#[test]
fn movupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 223], OperandSize::Qword)
}

#[test]
fn movupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 996814923, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 148, 83, 75, 48, 106, 59], OperandSize::Qword)
}

#[test]
fn movupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 240], OperandSize::Dword)
}

#[test]
fn movupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 1539026903, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 4, 205, 215, 175, 187, 91], OperandSize::Dword)
}

#[test]
fn movupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 222], OperandSize::Qword)
}

#[test]
fn movupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 2017649929, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 12, 189, 9, 229, 66, 120], OperandSize::Qword)
}

