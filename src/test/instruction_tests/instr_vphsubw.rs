use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 5, 225], OperandSize::Dword)
}

#[test]
fn vphsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 5, 38], OperandSize::Dword)
}

#[test]
fn vphsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 5, 247], OperandSize::Qword)
}

#[test]
fn vphsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 915943271, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 4, 117, 103, 47, 152, 54], OperandSize::Qword)
}

#[test]
fn vphsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 5, 200], OperandSize::Dword)
}

#[test]
fn vphsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 2141149061, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 5, 180, 82, 133, 87, 159, 127], OperandSize::Dword)
}

#[test]
fn vphsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 5, 233], OperandSize::Qword)
}

#[test]
fn vphsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 536998226, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 5, 164, 113, 82, 241, 1, 32], OperandSize::Qword)
}

