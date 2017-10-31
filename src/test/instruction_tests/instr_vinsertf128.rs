use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 24, 246, 97], OperandSize::Dword)
}

#[test]
fn vinsertf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 24, 28, 119, 2], OperandSize::Dword)
}

#[test]
fn vinsertf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 24, 244, 76], OperandSize::Qword)
}

#[test]
fn vinsertf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 24, 3, 96], OperandSize::Qword)
}

