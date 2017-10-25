use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 203], OperandSize::Dword)
}

#[test]
fn divss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 44959406, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 136, 174, 6, 174, 2], OperandSize::Dword)
}

#[test]
fn divss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 217], OperandSize::Qword)
}

#[test]
fn divss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 706545301, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 158, 149, 6, 29, 42], OperandSize::Qword)
}

