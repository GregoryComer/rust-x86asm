use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 24, 253, 63], OperandSize::Dword)
}

#[test]
fn vinsertf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1683178851, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 24, 164, 152, 99, 69, 83, 100, 40], OperandSize::Dword)
}

#[test]
fn vinsertf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 24, 243, 93], OperandSize::Qword)
}

#[test]
fn vinsertf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 24, 6, 43], OperandSize::Qword)
}

