use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 230], OperandSize::Dword)
}

#[test]
fn pshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1759691319, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 44, 133, 55, 194, 226, 104], OperandSize::Dword)
}

#[test]
fn pshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 247], OperandSize::Qword)
}

#[test]
fn pshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1026305703, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 20, 117, 167, 46, 44, 61], OperandSize::Qword)
}

#[test]
fn pshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 194], OperandSize::Dword)
}

#[test]
fn pshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 44, 139], OperandSize::Dword)
}

#[test]
fn pshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 255], OperandSize::Qword)
}

#[test]
fn pshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1137965567, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 140, 198, 255, 249, 211, 67], OperandSize::Qword)
}

