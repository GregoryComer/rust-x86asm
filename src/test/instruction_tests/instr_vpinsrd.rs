use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 34, 207, 39], OperandSize::Dword)
}

#[test]
fn vpinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1664947689, Some(OperandSize::Dword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 34, 44, 141, 233, 21, 61, 99, 93], OperandSize::Dword)
}

#[test]
fn vpinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 34, 198, 67], OperandSize::Qword)
}

#[test]
fn vpinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 34, 22, 104], OperandSize::Qword)
}

#[test]
fn vpinsrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 34, 202, 46], OperandSize::Dword)
}

#[test]
fn vpinsrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 34, 52, 201, 108], OperandSize::Dword)
}

#[test]
fn vpinsrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 5, 8, 34, 220, 75], OperandSize::Qword)
}

#[test]
fn vpinsrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1041062756, Some(OperandSize::Dword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 29, 0, 34, 156, 152, 100, 91, 13, 62, 9], OperandSize::Qword)
}

