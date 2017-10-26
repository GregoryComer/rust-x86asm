use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 223], OperandSize::Dword)
}

#[test]
fn movshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1673849045, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 132, 112, 213, 232, 196, 99], OperandSize::Dword)
}

#[test]
fn movshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 252], OperandSize::Qword)
}

#[test]
fn movshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RSI, 1238363193, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 190, 57, 236, 207, 73], OperandSize::Qword)
}

