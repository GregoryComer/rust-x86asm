use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 210], OperandSize::Dword)
}

#[test]
fn movups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 18], OperandSize::Dword)
}

#[test]
fn movups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 228], OperandSize::Qword)
}

#[test]
fn movups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1200435225, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 60, 245, 25, 48, 141, 71], OperandSize::Qword)
}

#[test]
fn movups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 209], OperandSize::Dword)
}

#[test]
fn movups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 681807968, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 172, 247, 96, 144, 163, 40], OperandSize::Dword)
}

#[test]
fn movups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 194], OperandSize::Qword)
}

#[test]
fn movups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 44, 178], OperandSize::Qword)
}

