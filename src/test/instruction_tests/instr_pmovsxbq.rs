use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 210], OperandSize::Dword)
}

#[test]
fn pmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 545252409, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 36, 69, 57, 228, 127, 32], OperandSize::Dword)
}

#[test]
fn pmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 235], OperandSize::Qword)
}

#[test]
fn pmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1642279168, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 60, 141, 0, 49, 227, 97], OperandSize::Qword)
}

