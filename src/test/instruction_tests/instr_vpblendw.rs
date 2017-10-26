use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 14, 228, 119], OperandSize::Dword)
}

#[test]
fn vpblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 14, 58, 116], OperandSize::Dword)
}

#[test]
fn vpblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 14, 229, 48], OperandSize::Qword)
}

#[test]
fn vpblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 14, 56, 6], OperandSize::Qword)
}

#[test]
fn vpblendw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 14, 246, 12], OperandSize::Dword)
}

#[test]
fn vpblendw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 14, 2, 70], OperandSize::Dword)
}

#[test]
fn vpblendw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 14, 215, 100], OperandSize::Qword)
}

#[test]
fn vpblendw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 347313729, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 14, 52, 77, 65, 150, 179, 20, 89], OperandSize::Qword)
}

