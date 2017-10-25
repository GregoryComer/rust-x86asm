use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 226], OperandSize::Word)
}

#[test]
fn verr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 33], OperandSize::Word)
}

#[test]
fn verr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 230], OperandSize::Dword)
}

#[test]
fn verr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 90], OperandSize::Dword)
}

#[test]
fn verr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 229], OperandSize::Qword)
}

#[test]
fn verr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectDisplaced(RBX, 420031692, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 163, 204, 44, 9, 25], OperandSize::Qword)
}

