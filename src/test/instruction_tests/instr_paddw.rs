use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 221], OperandSize::Dword)
}

#[test]
fn paddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1120646803, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 172, 155, 147, 182, 203, 66], OperandSize::Dword)
}

#[test]
fn paddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 203], OperandSize::Qword)
}

#[test]
fn paddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RBX, 2069348160, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 147, 64, 191, 87, 123], OperandSize::Qword)
}

#[test]
fn paddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 202], OperandSize::Dword)
}

#[test]
fn paddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1520529800, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 20, 85, 136, 113, 161, 90], OperandSize::Dword)
}

#[test]
fn paddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 245], OperandSize::Qword)
}

#[test]
fn paddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 28, 152], OperandSize::Qword)
}

