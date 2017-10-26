use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg8b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 21842, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 137, 82, 85], OperandSize::Word)
}

#[test]
fn cmpxchg8b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectDisplaced(ECX, 1428881613, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 137, 205, 0, 43, 85], OperandSize::Dword)
}

#[test]
fn cmpxchg8b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 12, 127], OperandSize::Qword)
}

