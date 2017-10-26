use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 213], OperandSize::Dword)
}

#[test]
fn unpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 240331441, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 44, 181, 177, 42, 83, 14], OperandSize::Dword)
}

#[test]
fn unpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 253], OperandSize::Qword)
}

#[test]
fn unpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1613043701, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 36, 85, 245, 23, 37, 96], OperandSize::Qword)
}

