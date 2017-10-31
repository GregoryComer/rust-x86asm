use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 239], OperandSize::Word)
}

#[test]
fn verw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectDisplaced(BX, 191, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 175, 191, 0], OperandSize::Word)
}

#[test]
fn verw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 234], OperandSize::Dword)
}

#[test]
fn verw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectDisplaced(EDI, 2083809611, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 175, 75, 105, 52, 124], OperandSize::Dword)
}

#[test]
fn verw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 237], OperandSize::Qword)
}

#[test]
fn verw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 43], OperandSize::Qword)
}

