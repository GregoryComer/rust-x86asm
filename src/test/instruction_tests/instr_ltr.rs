use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ltr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 218], OperandSize::Word)
}

#[test]
fn ltr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 26], OperandSize::Word)
}

#[test]
fn ltr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 219], OperandSize::Dword)
}

#[test]
fn ltr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectDisplaced(EDI, 929079463, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 159, 167, 160, 96, 55], OperandSize::Dword)
}

#[test]
fn ltr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 218], OperandSize::Qword)
}

#[test]
fn ltr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectDisplaced(RAX, 1907214098, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 152, 18, 199, 173, 113], OperandSize::Qword)
}

