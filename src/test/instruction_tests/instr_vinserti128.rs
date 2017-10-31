use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinserti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 56, 250, 13], OperandSize::Dword)
}

#[test]
fn vinserti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 56, 52, 70, 96], OperandSize::Dword)
}

#[test]
fn vinserti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 56, 201, 21], OperandSize::Qword)
}

#[test]
fn vinserti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 56, 19, 117], OperandSize::Qword)
}

