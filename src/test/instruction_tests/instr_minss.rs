use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 244], OperandSize::Dword)
}

#[test]
fn minss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 36, 211], OperandSize::Dword)
}

#[test]
fn minss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 226], OperandSize::Qword)
}

#[test]
fn minss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1211549296, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 60, 133, 112, 198, 54, 72], OperandSize::Qword)
}

