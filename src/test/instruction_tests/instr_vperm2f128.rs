use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2f128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 6, 200, 92], OperandSize::Dword)
}

#[test]
fn vperm2f128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 6, 20, 90, 126], OperandSize::Dword)
}

#[test]
fn vperm2f128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 6, 202, 24], OperandSize::Qword)
}

#[test]
fn vperm2f128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1855509765, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 6, 140, 143, 5, 213, 152, 110, 61], OperandSize::Qword)
}

