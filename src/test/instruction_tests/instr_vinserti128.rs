use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinserti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 56, 199, 12], OperandSize::Dword)
}

#[test]
fn vinserti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 332718065, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 56, 52, 125, 241, 223, 212, 19, 59], OperandSize::Dword)
}

#[test]
fn vinserti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 56, 228, 65], OperandSize::Qword)
}

#[test]
fn vinserti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 763698209, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 56, 184, 33, 28, 133, 45, 10], OperandSize::Qword)
}

