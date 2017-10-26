use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 201], OperandSize::Dword)
}

#[test]
fn psubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 20, 214], OperandSize::Dword)
}

#[test]
fn psubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 233], OperandSize::Qword)
}

#[test]
fn psubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 52, 222], OperandSize::Qword)
}

#[test]
fn psubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 215], OperandSize::Dword)
}

#[test]
fn psubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 874756480, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 132, 74, 128, 185, 35, 52], OperandSize::Dword)
}

#[test]
fn psubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 218], OperandSize::Qword)
}

#[test]
fn psubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 44, 79], OperandSize::Qword)
}

