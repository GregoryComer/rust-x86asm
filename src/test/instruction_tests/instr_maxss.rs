use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 227], OperandSize::Dword)
}

#[test]
fn maxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1656325262, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 148, 151, 142, 132, 185, 98], OperandSize::Dword)
}

#[test]
fn maxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 207], OperandSize::Qword)
}

#[test]
fn maxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 115226182, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 20, 85, 70, 54, 222, 6], OperandSize::Qword)
}

