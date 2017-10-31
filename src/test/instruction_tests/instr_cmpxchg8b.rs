use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg8b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 14963, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 136, 115, 58], OperandSize::Word)
}

#[test]
fn cmpxchg8b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 8], OperandSize::Dword)
}

#[test]
fn cmpxchg8b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 9], OperandSize::Qword)
}

