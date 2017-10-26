use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2i128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 70, 221, 64], OperandSize::Dword)
}

#[test]
fn vperm2i128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 886030486, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 70, 148, 214, 150, 192, 207, 52, 26], OperandSize::Dword)
}

#[test]
fn vperm2i128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 70, 224, 51], OperandSize::Qword)
}

#[test]
fn vperm2i128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RSI, 1077473657, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 70, 166, 121, 241, 56, 64, 28], OperandSize::Qword)
}

