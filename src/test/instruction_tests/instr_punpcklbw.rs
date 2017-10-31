use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 214], OperandSize::Dword)
}

#[test]
fn punpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(EDX, 524675824, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 162, 240, 234, 69, 31], OperandSize::Dword)
}

#[test]
fn punpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 230], OperandSize::Qword)
}

#[test]
fn punpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 2022615004, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 172, 209, 220, 167, 142, 120], OperandSize::Qword)
}

#[test]
fn punpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 218], OperandSize::Dword)
}

#[test]
fn punpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 1933834120, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 186, 136, 247, 67, 115], OperandSize::Dword)
}

#[test]
fn punpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 210], OperandSize::Qword)
}

#[test]
fn punpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 627726839, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 159, 247, 89, 106, 37], OperandSize::Qword)
}

