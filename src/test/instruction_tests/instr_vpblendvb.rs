use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 76, 202, 96], OperandSize::Dword)
}

#[test]
fn vpblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 772196767, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 76, 60, 221, 159, 201, 6, 46, 32], OperandSize::Dword)
}

#[test]
fn vpblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 76, 216, 32], OperandSize::Qword)
}

#[test]
fn vpblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2126132450, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 60, 125, 226, 52, 186, 126, 96], OperandSize::Qword)
}

#[test]
fn vpblendvb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 76, 228, 48], OperandSize::Dword)
}

#[test]
fn vpblendvb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1124495006, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 76, 139, 158, 110, 6, 67, 64], OperandSize::Dword)
}

#[test]
fn vpblendvb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 230, 112], OperandSize::Qword)
}

#[test]
fn vpblendvb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 2021031945, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 160, 9, 128, 118, 120, 80], OperandSize::Qword)
}

