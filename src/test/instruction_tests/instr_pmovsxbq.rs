use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 238], OperandSize::Dword)
}

#[test]
fn pmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1508611057, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 36, 189, 241, 147, 235, 89], OperandSize::Dword)
}

#[test]
fn pmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 236], OperandSize::Qword)
}

#[test]
fn pmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 583629018, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 134, 218, 120, 201, 34], OperandSize::Qword)
}

