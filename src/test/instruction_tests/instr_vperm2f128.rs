use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2f128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 6, 255, 75], OperandSize::Dword)
}

#[test]
fn vperm2f128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 6, 4, 144, 81], OperandSize::Dword)
}

#[test]
fn vperm2f128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 6, 201, 97], OperandSize::Qword)
}

#[test]
fn vperm2f128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 6, 60, 187, 88], OperandSize::Qword)
}

