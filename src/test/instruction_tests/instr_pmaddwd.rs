use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 205], OperandSize::Dword)
}

#[test]
fn pmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 60, 81], OperandSize::Dword)
}

#[test]
fn pmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 243], OperandSize::Qword)
}

#[test]
fn pmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 818765866, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 28, 141, 42, 96, 205, 48], OperandSize::Qword)
}

#[test]
fn pmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 204], OperandSize::Dword)
}

#[test]
fn pmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 1598490099, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 147, 243, 5, 71, 95], OperandSize::Dword)
}

#[test]
fn pmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 245], OperandSize::Qword)
}

#[test]
fn pmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 28, 142], OperandSize::Qword)
}

