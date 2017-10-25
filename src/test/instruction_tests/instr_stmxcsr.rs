use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1527996152, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 93, 248, 94, 19, 91], OperandSize::Dword)
}

#[test]
fn stmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectDisplaced(RDI, 309535897, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 159, 153, 36, 115, 18], OperandSize::Qword)
}

