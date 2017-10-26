use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 238], OperandSize::Dword)
}

#[test]
fn movsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1572344751, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 60, 93, 175, 19, 184, 93], OperandSize::Dword)
}

#[test]
fn movsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 241], OperandSize::Qword)
}

#[test]
fn movsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 4, 243], OperandSize::Qword)
}

