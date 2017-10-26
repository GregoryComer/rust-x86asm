use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 32, 199, 63], OperandSize::Dword)
}

#[test]
fn vpinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 93022422, Some(OperandSize::Byte), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 32, 28, 189, 214, 104, 139, 5, 126], OperandSize::Dword)
}

#[test]
fn vpinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 32, 250, 31], OperandSize::Qword)
}

#[test]
fn vpinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 32, 33, 4], OperandSize::Qword)
}

#[test]
fn vpinsrb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EBX)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 32, 243, 81], OperandSize::Dword)
}

#[test]
fn vpinsrb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1738939985, Some(OperandSize::Byte), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 32, 180, 242, 81, 30, 166, 103, 74], OperandSize::Dword)
}

#[test]
fn vpinsrb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 5, 0, 32, 204, 85], OperandSize::Qword)
}

#[test]
fn vpinsrb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 105, 32, 7, 102], OperandSize::Qword)
}

