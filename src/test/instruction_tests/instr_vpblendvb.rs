use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 76, 231, 32], OperandSize::Dword)
}

#[test]
fn vpblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1122809459, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 76, 156, 88, 115, 182, 236, 66, 80], OperandSize::Dword)
}

#[test]
fn vpblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 76, 244, 112], OperandSize::Qword)
}

#[test]
fn vpblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 76, 44, 142, 112], OperandSize::Qword)
}

#[test]
fn vpblendvb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 229, 96], OperandSize::Dword)
}

#[test]
fn vpblendvb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 653811068, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 12, 125, 124, 93, 248, 38, 32], OperandSize::Dword)
}

#[test]
fn vpblendvb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 219, 96], OperandSize::Qword)
}

#[test]
fn vpblendvb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 2069005562, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 76, 164, 114, 250, 132, 82, 123, 16], OperandSize::Qword)
}

