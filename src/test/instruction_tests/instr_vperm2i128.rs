use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2i128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 70, 236, 64], OperandSize::Dword)
}

#[test]
fn vperm2i128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 616802883, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 70, 184, 67, 170, 195, 36, 13], OperandSize::Dword)
}

#[test]
fn vperm2i128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 70, 211, 17], OperandSize::Qword)
}

#[test]
fn vperm2i128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 70, 46, 103], OperandSize::Qword)
}

