use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 239], OperandSize::Dword)
}

#[test]
fn pmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 60, 208], OperandSize::Dword)
}

#[test]
fn pmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 213], OperandSize::Qword)
}

#[test]
fn pmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1977712875, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 156, 80, 235, 128, 225, 117], OperandSize::Qword)
}

