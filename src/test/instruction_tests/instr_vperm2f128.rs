use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2f128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 6, 219, 7], OperandSize::Dword)
}

#[test]
fn vperm2f128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 6, 42, 101], OperandSize::Dword)
}

#[test]
fn vperm2f128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 6, 253, 45], OperandSize::Qword)
}

#[test]
fn vperm2f128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDI, 131490697, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 6, 167, 137, 99, 214, 7, 80], OperandSize::Qword)
}

