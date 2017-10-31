use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 192], OperandSize::Dword)
}

#[test]
fn punpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 47], OperandSize::Dword)
}

#[test]
fn punpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 247], OperandSize::Qword)
}

#[test]
fn punpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 20, 185], OperandSize::Qword)
}

#[test]
fn punpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 246], OperandSize::Dword)
}

#[test]
fn punpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 52, 191], OperandSize::Dword)
}

#[test]
fn punpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 237], OperandSize::Qword)
}

#[test]
fn punpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 267721811, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 186, 83, 28, 245, 15], OperandSize::Qword)
}

