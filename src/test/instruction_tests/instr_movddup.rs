use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 196], OperandSize::Dword)
}

#[test]
fn movddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 352041797, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 132, 194, 69, 187, 251, 20], OperandSize::Dword)
}

#[test]
fn movddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 254], OperandSize::Qword)
}

#[test]
fn movddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1712526665, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 20, 205, 73, 21, 19, 102], OperandSize::Qword)
}

