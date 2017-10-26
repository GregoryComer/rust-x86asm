use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 12, 234, 99], OperandSize::Dword)
}

#[test]
fn vblendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 44, 91, 103], OperandSize::Dword)
}

#[test]
fn vblendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 228, 107], OperandSize::Qword)
}

#[test]
fn vblendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 33, 43], OperandSize::Qword)
}

#[test]
fn vblendps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 12, 215, 23], OperandSize::Dword)
}

#[test]
fn vblendps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 156784337, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 12, 151, 209, 86, 88, 9, 90], OperandSize::Dword)
}

#[test]
fn vblendps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 12, 208, 88], OperandSize::Qword)
}

#[test]
fn vblendps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RSI, 208325903, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 12, 134, 15, 205, 106, 12, 93], OperandSize::Qword)
}

