use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 242], OperandSize::Dword)
}

#[test]
fn vtestps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 48], OperandSize::Dword)
}

#[test]
fn vtestps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 202], OperandSize::Qword)
}

#[test]
fn vtestps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 12, 115], OperandSize::Qword)
}

#[test]
fn vtestps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 223], OperandSize::Dword)
}

#[test]
fn vtestps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 577551961, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 28, 133, 89, 190, 108, 34], OperandSize::Dword)
}

#[test]
fn vtestps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 232], OperandSize::Qword)
}

#[test]
fn vtestps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 36, 73], OperandSize::Qword)
}

