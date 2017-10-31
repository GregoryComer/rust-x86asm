use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 228], OperandSize::Word)
}

#[test]
fn verr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Memory(9796, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 38, 68, 38], OperandSize::Word)
}

#[test]
fn verr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 226], OperandSize::Dword)
}

#[test]
fn verr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 210], OperandSize::Dword)
}

#[test]
fn verr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 226], OperandSize::Qword)
}

#[test]
fn verr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 95], OperandSize::Qword)
}

