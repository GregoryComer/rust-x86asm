use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 217], OperandSize::Dword)
}

#[test]
fn addss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 956293813, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 52, 117, 181, 226, 255, 56], OperandSize::Dword)
}

#[test]
fn addss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 230], OperandSize::Qword)
}

#[test]
fn addss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 574983088, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 180, 79, 176, 139, 69, 34], OperandSize::Qword)
}

