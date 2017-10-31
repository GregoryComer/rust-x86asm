use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 8, 248], OperandSize::Dword)
}

#[test]
fn vpsignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 8, 4, 83], OperandSize::Dword)
}

#[test]
fn vpsignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 8, 248], OperandSize::Qword)
}

#[test]
fn vpsignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 8, 12, 72], OperandSize::Qword)
}

#[test]
fn vpsignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 8, 215], OperandSize::Dword)
}

#[test]
fn vpsignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 8, 11], OperandSize::Dword)
}

#[test]
fn vpsignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 8, 222], OperandSize::Qword)
}

#[test]
fn vpsignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 8, 12, 115], OperandSize::Qword)
}

