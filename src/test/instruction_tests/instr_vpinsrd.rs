use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 34, 210, 31], OperandSize::Dword)
}

#[test]
fn vpinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 34, 36, 82, 108], OperandSize::Dword)
}

#[test]
fn vpinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 34, 201, 110], OperandSize::Qword)
}

#[test]
fn vpinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 1935395233, Some(OperandSize::Dword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 34, 151, 161, 201, 91, 115, 63], OperandSize::Qword)
}

#[test]
fn vpinsrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 34, 225, 64], OperandSize::Dword)
}

#[test]
fn vpinsrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 34, 44, 80, 1], OperandSize::Dword)
}

#[test]
fn vpinsrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 0, 34, 230, 45], OperandSize::Qword)
}

#[test]
fn vpinsrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RDX, 409267173, Some(OperandSize::Dword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 93, 0, 34, 146, 229, 235, 100, 24, 22], OperandSize::Qword)
}

