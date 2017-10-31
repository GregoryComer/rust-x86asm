use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 32, 206, 61], OperandSize::Dword)
}

#[test]
fn vpinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 501146279, Some(OperandSize::Byte), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 28, 93, 167, 226, 222, 29, 86], OperandSize::Dword)
}

#[test]
fn vpinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 194, 103], OperandSize::Qword)
}

#[test]
fn vpinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Byte), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 32, 20, 249, 17], OperandSize::Qword)
}

#[test]
fn vpinsrb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 32, 209, 89], OperandSize::Dword)
}

#[test]
fn vpinsrb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1440690725, Some(OperandSize::Byte), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 32, 145, 37, 50, 223, 85, 95], OperandSize::Dword)
}

#[test]
fn vpinsrb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 13, 0, 32, 225, 0], OperandSize::Qword)
}

#[test]
fn vpinsrb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 115, 117, 0, 32, 42, 92], OperandSize::Qword)
}

