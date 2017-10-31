use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 219, 238], OperandSize::Dword)
}

#[test]
fn vpand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 148093265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 219, 132, 134, 81, 185, 211, 8], OperandSize::Dword)
}

#[test]
fn vpand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 219, 202], OperandSize::Qword)
}

#[test]
fn vpand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 219, 44, 182], OperandSize::Qword)
}

#[test]
fn vpand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 219, 205], OperandSize::Dword)
}

#[test]
fn vpand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 507950815, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 219, 52, 149, 223, 182, 70, 30], OperandSize::Dword)
}

#[test]
fn vpand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 219, 230], OperandSize::Qword)
}

#[test]
fn vpand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1164654257, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 219, 4, 245, 177, 54, 107, 69], OperandSize::Qword)
}

