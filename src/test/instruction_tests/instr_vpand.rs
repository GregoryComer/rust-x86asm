use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 219, 218], OperandSize::Dword)
}

#[test]
fn vpand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1021168160, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 219, 4, 149, 32, 202, 221, 60], OperandSize::Dword)
}

#[test]
fn vpand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 219, 239], OperandSize::Qword)
}

#[test]
fn vpand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 629541460, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 219, 52, 181, 84, 10, 134, 37], OperandSize::Qword)
}

#[test]
fn vpand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 219, 217], OperandSize::Dword)
}

#[test]
fn vpand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 2118450534, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 219, 137, 102, 253, 68, 126], OperandSize::Dword)
}

#[test]
fn vpand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 219, 244], OperandSize::Qword)
}

#[test]
fn vpand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1058832332, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 219, 60, 141, 204, 127, 28, 63], OperandSize::Qword)
}

