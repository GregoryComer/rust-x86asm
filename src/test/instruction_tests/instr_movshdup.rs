use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 252], OperandSize::Dword)
}

#[test]
fn movshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 52, 126], OperandSize::Dword)
}

#[test]
fn movshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 195], OperandSize::Qword)
}

#[test]
fn movshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1744048788, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 176, 148, 18, 244, 103], OperandSize::Qword)
}

