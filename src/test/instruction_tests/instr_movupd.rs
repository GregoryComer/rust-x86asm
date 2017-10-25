use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 221], OperandSize::Dword)
}

#[test]
fn movupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1986167788, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 180, 153, 236, 131, 98, 118], OperandSize::Dword)
}

#[test]
fn movupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 209], OperandSize::Qword)
}

#[test]
fn movupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 16], OperandSize::Qword)
}

#[test]
fn movupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 206], OperandSize::Dword)
}

#[test]
fn movupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 52, 203], OperandSize::Dword)
}

#[test]
fn movupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 225], OperandSize::Qword)
}

#[test]
fn movupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1281924877, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 4, 189, 13, 159, 104, 76], OperandSize::Qword)
}

