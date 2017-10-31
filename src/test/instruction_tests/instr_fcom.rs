use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(SI, 184, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 148, 184, 0], OperandSize::Word)
}

#[test]
fn fcom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 184], OperandSize::Dword)
}

#[test]
fn fcom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 184], OperandSize::Qword)
}

#[test]
fn fcom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 209], OperandSize::Word)
}

#[test]
fn fcom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 213], OperandSize::Dword)
}

#[test]
fn fcom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 214], OperandSize::Qword)
}

#[test]
fn fcom_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 16], OperandSize::Word)
}

#[test]
fn fcom_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 580534853, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 148, 194, 69, 66, 154, 34], OperandSize::Dword)
}

#[test]
fn fcom_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 19], OperandSize::Qword)
}

