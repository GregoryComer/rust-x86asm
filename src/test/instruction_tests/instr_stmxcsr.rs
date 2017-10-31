use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1183213273, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 213, 217, 102, 134, 70], OperandSize::Dword)
}

#[test]
fn stmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 377160908, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 149, 204, 4, 123, 22], OperandSize::Qword)
}

