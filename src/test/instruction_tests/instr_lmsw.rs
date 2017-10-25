use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lmsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 247], OperandSize::Word)
}

#[test]
fn lmsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectDisplaced(BX, 111, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 119, 111], OperandSize::Word)
}

#[test]
fn lmsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 242], OperandSize::Dword)
}

#[test]
fn lmsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 50], OperandSize::Dword)
}

#[test]
fn lmsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 247], OperandSize::Qword)
}

#[test]
fn lmsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1786496245, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 180, 250, 245, 196, 123, 106], OperandSize::Qword)
}

