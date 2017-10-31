use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 233], OperandSize::Dword)
}

#[test]
fn sqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 420850463, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 52, 245, 31, 171, 21, 25], OperandSize::Dword)
}

#[test]
fn sqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 244], OperandSize::Qword)
}

#[test]
fn sqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 131511281, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 164, 129, 241, 179, 214, 7], OperandSize::Qword)
}

