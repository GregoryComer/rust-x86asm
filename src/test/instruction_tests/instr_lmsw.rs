use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lmsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 241], OperandSize::Word)
}

#[test]
fn lmsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52], OperandSize::Word)
}

#[test]
fn lmsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 245], OperandSize::Dword)
}

#[test]
fn lmsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledDisplaced(ECX, Two, 2074855014, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52, 77, 102, 198, 171, 123], OperandSize::Dword)
}

#[test]
fn lmsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 242], OperandSize::Qword)
}

#[test]
fn lmsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1686553707, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52, 181, 107, 196, 134, 100], OperandSize::Qword)
}

