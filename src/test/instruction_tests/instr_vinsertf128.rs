use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 24, 234, 58], OperandSize::Dword)
}

#[test]
fn vinsertf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 1228449133, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 24, 128, 109, 165, 56, 73, 19], OperandSize::Dword)
}

#[test]
fn vinsertf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 24, 233, 90], OperandSize::Qword)
}

#[test]
fn vinsertf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 24, 50, 43], OperandSize::Qword)
}

