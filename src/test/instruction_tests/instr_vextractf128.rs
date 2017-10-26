use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 239, 36], OperandSize::Dword)
}

#[test]
fn vextractf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 4, 130, 40], OperandSize::Dword)
}

#[test]
fn vextractf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 194, 6], OperandSize::Qword)
}

#[test]
fn vextractf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 52, 70, 95], OperandSize::Qword)
}

