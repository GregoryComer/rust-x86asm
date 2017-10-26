use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 34, 225, 1], OperandSize::Dword)
}

#[test]
fn vpinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 34, 59, 7], OperandSize::Dword)
}

#[test]
fn vpinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EBP)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 34, 237, 1], OperandSize::Qword)
}

#[test]
fn vpinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 1742541586, Some(OperandSize::Dword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 34, 175, 18, 19, 221, 103, 97], OperandSize::Qword)
}

#[test]
fn vpinsrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 34, 239, 96], OperandSize::Dword)
}

#[test]
fn vpinsrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2027031776, Some(OperandSize::Dword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 34, 60, 77, 224, 12, 210, 120, 39], OperandSize::Dword)
}

#[test]
fn vpinsrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 45, 0, 34, 249, 82], OperandSize::Qword)
}

#[test]
fn vpinsrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RCX, 817080739, Some(OperandSize::Dword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 29, 0, 34, 153, 163, 169, 179, 48, 68], OperandSize::Qword)
}

