use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(BP, 59, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 86, 59], OperandSize::Word)
}

#[test]
fn fcom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 16], OperandSize::Dword)
}

#[test]
fn fcom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 182], OperandSize::Qword)
}

#[test]
fn fcom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 209], OperandSize::Word)
}

#[test]
fn fcom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 212], OperandSize::Dword)
}

#[test]
fn fcom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 210], OperandSize::Qword)
}

#[test]
fn fcom_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 23], OperandSize::Word)
}

#[test]
fn fcom_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1472097496, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 20, 125, 216, 108, 190, 87], OperandSize::Dword)
}

#[test]
fn fcom_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 19], OperandSize::Qword)
}

