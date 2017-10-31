use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 249, 112], OperandSize::Dword)
}

#[test]
fn vpblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1938073083, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 76, 129, 251, 165, 132, 115, 0], OperandSize::Dword)
}

#[test]
fn vpblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 208, 32], OperandSize::Qword)
}

#[test]
fn vpblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 50, 48], OperandSize::Qword)
}

#[test]
fn vpblendvb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 76, 199, 96], OperandSize::Dword)
}

#[test]
fn vpblendvb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 867277289, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 76, 143, 233, 153, 177, 51, 0], OperandSize::Dword)
}

#[test]
fn vpblendvb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 227, 16], OperandSize::Qword)
}

#[test]
fn vpblendvb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 71153082, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 76, 52, 77, 186, 181, 61, 4, 64], OperandSize::Qword)
}

