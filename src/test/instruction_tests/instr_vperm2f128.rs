use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2f128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 6, 193, 72], OperandSize::Dword)
}

#[test]
fn vperm2f128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 966779311, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 6, 156, 254, 175, 225, 159, 57, 75], OperandSize::Dword)
}

#[test]
fn vperm2f128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 6, 203, 66], OperandSize::Qword)
}

#[test]
fn vperm2f128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 334448042, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 6, 44, 181, 170, 69, 239, 19, 80], OperandSize::Qword)
}

