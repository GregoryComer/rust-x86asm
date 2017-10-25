use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 232], OperandSize::Dword)
}

#[test]
fn minss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 12, 83], OperandSize::Dword)
}

#[test]
fn minss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 241], OperandSize::Qword)
}

#[test]
fn minss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 2089367411, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 156, 211, 115, 55, 137, 124], OperandSize::Qword)
}

