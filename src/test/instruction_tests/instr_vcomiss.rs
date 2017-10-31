use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 239], OperandSize::Dword)
}

#[test]
fn vcomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1628934667, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 12, 205, 11, 146, 23, 97], OperandSize::Dword)
}

#[test]
fn vcomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 239], OperandSize::Qword)
}

#[test]
fn vcomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 44, 114], OperandSize::Qword)
}

#[test]
fn vcomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 200], OperandSize::Dword)
}

#[test]
fn vcomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 44, 203], OperandSize::Dword)
}

#[test]
fn vcomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 124, 24, 47, 250], OperandSize::Qword)
}

#[test]
fn vcomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM29)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 47, 42], OperandSize::Qword)
}

