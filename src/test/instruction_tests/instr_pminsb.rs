use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 235], OperandSize::Dword)
}

#[test]
fn pminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1216151992, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 4, 253, 184, 1, 125, 72], OperandSize::Dword)
}

#[test]
fn pminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 246], OperandSize::Qword)
}

#[test]
fn pminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 20, 89], OperandSize::Qword)
}

