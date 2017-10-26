use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 64, 200, 25], OperandSize::Dword)
}

#[test]
fn vdpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 47, 113], OperandSize::Dword)
}

#[test]
fn vdpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 234, 48], OperandSize::Qword)
}

#[test]
fn vdpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 545291046, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 64, 190, 38, 123, 128, 32, 71], OperandSize::Qword)
}

#[test]
fn vdpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 64, 199, 126], OperandSize::Dword)
}

#[test]
fn vdpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2101043974, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 64, 44, 141, 6, 99, 59, 125, 67], OperandSize::Dword)
}

#[test]
fn vdpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 64, 237, 83], OperandSize::Qword)
}

#[test]
fn vdpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 64, 28, 70, 42], OperandSize::Qword)
}

