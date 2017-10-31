use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 32, 246, 123], OperandSize::Dword)
}

#[test]
fn vpinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Byte), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 32, 60, 135, 65], OperandSize::Dword)
}

#[test]
fn vpinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 239, 78], OperandSize::Qword)
}

#[test]
fn vpinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Byte), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 32, 28, 72, 91], OperandSize::Qword)
}

#[test]
fn vpinsrb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(EBP)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 32, 197, 116], OperandSize::Dword)
}

#[test]
fn vpinsrb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2118254638, Some(OperandSize::Byte), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 32, 20, 141, 46, 0, 66, 126, 69], OperandSize::Dword)
}

#[test]
fn vpinsrb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 33, 32, 198, 27], OperandSize::Qword)
}

#[test]
fn vpinsrb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1013918495, Some(OperandSize::Byte), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 65, 32, 12, 77, 31, 43, 111, 60, 102], OperandSize::Qword)
}

