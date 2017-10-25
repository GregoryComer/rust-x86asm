use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 247], OperandSize::Dword)
}

#[test]
fn vtestpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 1446890927, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 177, 175, 205, 61, 86], OperandSize::Dword)
}

#[test]
fn vtestpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 218], OperandSize::Qword)
}

#[test]
fn vtestpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1002053931, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 60, 133, 43, 33, 186, 59], OperandSize::Qword)
}

#[test]
fn vtestpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 213], OperandSize::Dword)
}

#[test]
fn vtestpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1632725256, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 188, 250, 8, 105, 81, 97], OperandSize::Dword)
}

#[test]
fn vtestpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 226], OperandSize::Qword)
}

#[test]
fn vtestpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RSI, 1388351187, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 166, 211, 142, 192, 82], OperandSize::Qword)
}

