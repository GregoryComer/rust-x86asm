use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 217], OperandSize::Dword)
}

#[test]
fn pmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 1717379155, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 184, 83, 32, 93, 102], OperandSize::Dword)
}

#[test]
fn pmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 231], OperandSize::Qword)
}

#[test]
fn pmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 356383492, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 44, 133, 4, 251, 61, 21], OperandSize::Qword)
}

