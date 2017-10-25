use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinserti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 56, 214, 12], OperandSize::Dword)
}

#[test]
fn vinserti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 56, 42, 42], OperandSize::Dword)
}

#[test]
fn vinserti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 56, 232, 4], OperandSize::Qword)
}

#[test]
fn vinserti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 330274930, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 56, 180, 79, 114, 152, 175, 19, 20], OperandSize::Qword)
}

