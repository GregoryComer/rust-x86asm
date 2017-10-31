use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ltr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 220], OperandSize::Word)
}

#[test]
fn ltr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 26, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 89, 26], OperandSize::Word)
}

#[test]
fn ltr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 223], OperandSize::Dword)
}

#[test]
fn ltr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 143], OperandSize::Dword)
}

#[test]
fn ltr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 221], OperandSize::Qword)
}

#[test]
fn ltr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 79], OperandSize::Qword)
}

