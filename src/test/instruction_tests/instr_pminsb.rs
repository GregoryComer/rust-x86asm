use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 197], OperandSize::Dword)
}

#[test]
fn pminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1152019552, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 12, 85, 96, 108, 170, 68], OperandSize::Dword)
}

#[test]
fn pminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 201], OperandSize::Qword)
}

#[test]
fn pminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 36, 128], OperandSize::Qword)
}

