use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 237], OperandSize::Dword)
}

#[test]
fn xorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1557660650, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 44, 213, 234, 3, 216, 92], OperandSize::Dword)
}

#[test]
fn xorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 250], OperandSize::Qword)
}

#[test]
fn xorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 555497375, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 164, 120, 159, 55, 28, 33], OperandSize::Qword)
}

