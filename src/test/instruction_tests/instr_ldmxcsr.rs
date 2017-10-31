use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectDisplaced(ECX, 892062410, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 145, 202, 202, 43, 53], OperandSize::Dword)
}

#[test]
fn ldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectDisplaced(RDI, 1418775791, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 151, 239, 204, 144, 84], OperandSize::Qword)
}

