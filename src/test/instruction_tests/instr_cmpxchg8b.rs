use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg8b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(Indirect(SI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 12], OperandSize::Word)
}

#[test]
fn cmpxchg8b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1759335804, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 140, 142, 124, 85, 221, 104], OperandSize::Dword)
}

#[test]
fn cmpxchg8b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectDisplaced(RDX, 76495837, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 138, 221, 59, 143, 4], OperandSize::Qword)
}

