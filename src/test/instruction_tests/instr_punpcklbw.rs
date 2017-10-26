use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 210], OperandSize::Dword)
}

#[test]
fn punpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 553053844, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 52, 157, 148, 238, 246, 32], OperandSize::Dword)
}

#[test]
fn punpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 238], OperandSize::Qword)
}

#[test]
fn punpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RDX, 946062122, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 138, 42, 195, 99, 56], OperandSize::Qword)
}

#[test]
fn punpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 234], OperandSize::Dword)
}

#[test]
fn punpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 1583239073, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 130, 161, 79, 94, 94], OperandSize::Dword)
}

#[test]
fn punpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 200], OperandSize::Qword)
}

#[test]
fn punpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1860773922, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 4, 125, 34, 40, 233, 110], OperandSize::Qword)
}

