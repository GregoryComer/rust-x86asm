use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 203], OperandSize::Dword)
}

#[test]
fn pmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 60, 191], OperandSize::Dword)
}

#[test]
fn pmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 213], OperandSize::Qword)
}

#[test]
fn pmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1904604327, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 132, 81, 167, 244, 133, 113], OperandSize::Qword)
}

#[test]
fn pmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 209], OperandSize::Dword)
}

#[test]
fn pmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 52, 222], OperandSize::Dword)
}

#[test]
fn pmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 231], OperandSize::Qword)
}

#[test]
fn pmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1168674817, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 148, 89, 1, 144, 168, 69], OperandSize::Qword)
}

