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
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 1900282525, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 178, 157, 2, 68, 113], OperandSize::Dword)
}

#[test]
fn movshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 232], OperandSize::Qword)
}

#[test]
fn movshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1050790665, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 185, 9, 203, 161, 62], OperandSize::Qword)
}

