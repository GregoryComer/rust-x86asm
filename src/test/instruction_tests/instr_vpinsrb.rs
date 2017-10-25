use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 255, 52], OperandSize::Dword)
}

#[test]
fn vpinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 136252772, Some(OperandSize::Byte), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 32, 180, 74, 100, 13, 31, 8, 17], OperandSize::Dword)
}

#[test]
fn vpinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EBP)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 32, 253, 22], OperandSize::Qword)
}

#[test]
fn vpinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Byte), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 32, 52, 177, 62], OperandSize::Qword)
}

#[test]
fn vpinsrb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(EDX)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 32, 242, 63], OperandSize::Dword)
}

#[test]
fn vpinsrb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 903910790, Some(OperandSize::Byte), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 32, 144, 134, 149, 224, 53, 26], OperandSize::Dword)
}

#[test]
fn vpinsrb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(EBX)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 69, 8, 32, 211, 99], OperandSize::Qword)
}

#[test]
fn vpinsrb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1127624655, Some(OperandSize::Byte), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 4, 85, 207, 47, 54, 67, 107], OperandSize::Qword)
}

