use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectDisplaced(ECX, 1101853158, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 153, 230, 241, 172, 65], OperandSize::Dword)
}

#[test]
fn stmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 64922495, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 77, 127, 163, 222, 3], OperandSize::Qword)
}

