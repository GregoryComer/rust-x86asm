use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20], OperandSize::Word)
}

#[test]
fn fcom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 90], OperandSize::Dword)
}

#[test]
fn fcom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 179], OperandSize::Qword)
}

#[test]
fn fcom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 213], OperandSize::Word)
}

#[test]
fn fcom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 209], OperandSize::Dword)
}

#[test]
fn fcom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 212], OperandSize::Qword)
}

#[test]
fn fcom_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 118, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 82, 118], OperandSize::Word)
}

#[test]
fn fcom_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(ESI, 800640856, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 150, 88, 207, 184, 47], OperandSize::Dword)
}

#[test]
fn fcom_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1209756898, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 20, 221, 226, 108, 27, 72], OperandSize::Qword)
}

