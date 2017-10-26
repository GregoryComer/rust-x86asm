use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectDisplaced(EAX, 642333352, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 144, 168, 58, 73, 38], OperandSize::Dword)
}

#[test]
fn ldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 23], OperandSize::Qword)
}

