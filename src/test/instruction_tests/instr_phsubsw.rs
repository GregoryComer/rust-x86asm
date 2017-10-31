use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 202], OperandSize::Dword)
}

#[test]
fn phsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 36206686, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 172, 74, 94, 120, 40, 2], OperandSize::Dword)
}

#[test]
fn phsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 232], OperandSize::Qword)
}

#[test]
fn phsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 4, 70], OperandSize::Qword)
}

#[test]
fn phsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 248], OperandSize::Dword)
}

#[test]
fn phsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 12, 115], OperandSize::Dword)
}

#[test]
fn phsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 225], OperandSize::Qword)
}

#[test]
fn phsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 28, 208], OperandSize::Qword)
}

