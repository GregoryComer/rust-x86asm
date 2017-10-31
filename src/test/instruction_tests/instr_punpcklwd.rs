use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 210], OperandSize::Dword)
}

#[test]
fn punpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 26], OperandSize::Dword)
}

#[test]
fn punpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 231], OperandSize::Qword)
}

#[test]
fn punpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1555764297, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 36, 149, 73, 20, 187, 92], OperandSize::Qword)
}

#[test]
fn punpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 211], OperandSize::Dword)
}

#[test]
fn punpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 28, 155], OperandSize::Dword)
}

#[test]
fn punpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 237], OperandSize::Qword)
}

#[test]
fn punpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 14], OperandSize::Qword)
}

