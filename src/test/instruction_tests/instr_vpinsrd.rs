use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 34, 238, 42], OperandSize::Dword)
}

#[test]
fn vpinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 34, 20, 159, 34], OperandSize::Dword)
}

#[test]
fn vpinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 34, 206, 0], OperandSize::Qword)
}

#[test]
fn vpinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 34, 4, 193, 93], OperandSize::Qword)
}

#[test]
fn vpinsrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 34, 234, 68], OperandSize::Dword)
}

#[test]
fn vpinsrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 34, 20, 219, 2], OperandSize::Dword)
}

#[test]
fn vpinsrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 57, 34, 244, 45], OperandSize::Qword)
}

#[test]
fn vpinsrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 113, 34, 52, 144, 33], OperandSize::Qword)
}

