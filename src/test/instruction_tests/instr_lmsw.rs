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
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 28803, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 178, 131, 112], OperandSize::Word)
}

#[test]
fn lmsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 243], OperandSize::Dword)
}

#[test]
fn lmsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1192761142, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52, 125, 54, 23, 24, 71], OperandSize::Dword)
}

#[test]
fn lmsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 246], OperandSize::Qword)
}

#[test]
fn lmsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 738691274, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52, 245, 202, 136, 7, 44], OperandSize::Qword)
}

