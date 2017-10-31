use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 244222251, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 148, 73, 43, 137, 142, 14], OperandSize::Dword)
}

#[test]
fn movlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 20, 248], OperandSize::Qword)
}

#[test]
fn movlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 60, 113], OperandSize::Dword)
}

#[test]
fn movlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 28, 246], OperandSize::Qword)
}

