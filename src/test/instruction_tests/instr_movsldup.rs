use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 216], OperandSize::Dword)
}

#[test]
fn movsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 36, 73], OperandSize::Dword)
}

#[test]
fn movsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 229], OperandSize::Qword)
}

#[test]
fn movsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1800840479, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 20, 221, 31, 165, 86, 107], OperandSize::Qword)
}

