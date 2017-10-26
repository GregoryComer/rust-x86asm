use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinserti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 56, 254, 84], OperandSize::Dword)
}

#[test]
fn vinserti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 56, 52, 122, 78], OperandSize::Dword)
}

#[test]
fn vinserti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 56, 221, 62], OperandSize::Qword)
}

#[test]
fn vinserti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1804991764, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 56, 156, 195, 20, 253, 149, 107, 5], OperandSize::Qword)
}

