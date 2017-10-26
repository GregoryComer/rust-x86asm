use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lmsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 243], OperandSize::Word)
}

#[test]
fn lmsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 53], OperandSize::Word)
}

#[test]
fn lmsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 247], OperandSize::Dword)
}

#[test]
fn lmsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1838620308, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 180, 113, 148, 30, 151, 109], OperandSize::Dword)
}

#[test]
fn lmsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 242], OperandSize::Qword)
}

#[test]
fn lmsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 54], OperandSize::Qword)
}

