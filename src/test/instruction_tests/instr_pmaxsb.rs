use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 249], OperandSize::Dword)
}

#[test]
fn pmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 1780709620, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 166, 244, 120, 35, 106], OperandSize::Dword)
}

#[test]
fn pmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 220], OperandSize::Qword)
}

#[test]
fn pmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1125055295, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 139, 63, 251, 14, 67], OperandSize::Qword)
}

