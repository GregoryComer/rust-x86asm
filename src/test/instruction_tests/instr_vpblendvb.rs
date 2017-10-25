use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 76, 234, 64], OperandSize::Dword)
}

#[test]
fn vpblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 44, 186, 96], OperandSize::Dword)
}

#[test]
fn vpblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 76, 219, 0], OperandSize::Qword)
}

#[test]
fn vpblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1844713356, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 76, 132, 210, 140, 23, 244, 109, 32], OperandSize::Qword)
}

#[test]
fn vpblendvb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 76, 224, 80], OperandSize::Dword)
}

#[test]
fn vpblendvb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 874772692, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 76, 20, 221, 212, 248, 35, 52, 32], OperandSize::Dword)
}

#[test]
fn vpblendvb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 215, 0], OperandSize::Qword)
}

#[test]
fn vpblendvb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 36, 206, 32], OperandSize::Qword)
}

