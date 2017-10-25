use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstors_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 29], OperandSize::Word)
}

#[test]
fn xrstors_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 647110028, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 156, 129, 140, 29, 146, 38], OperandSize::Dword)
}

#[test]
fn xrstors_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 811200519, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 156, 86, 7, 240, 89, 48], OperandSize::Qword)
}

