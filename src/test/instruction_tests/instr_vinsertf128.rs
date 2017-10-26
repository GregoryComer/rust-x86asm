use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 24, 223, 65], OperandSize::Dword)
}

#[test]
fn vinsertf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 24, 12, 86, 26], OperandSize::Dword)
}

#[test]
fn vinsertf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 24, 230, 5], OperandSize::Qword)
}

#[test]
fn vinsertf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RCX, 1599166380, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 24, 161, 172, 87, 81, 95, 94], OperandSize::Qword)
}

