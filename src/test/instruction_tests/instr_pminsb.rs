use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 223], OperandSize::Dword)
}

#[test]
fn pminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EAX, 1779304913, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 144, 209, 9, 14, 106], OperandSize::Dword)
}

#[test]
fn pminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 201], OperandSize::Qword)
}

#[test]
fn pminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1699663306, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 188, 223, 202, 205, 78, 101], OperandSize::Qword)
}

