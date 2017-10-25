use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 2003666992, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 148, 250, 48, 136, 109, 119], OperandSize::Dword)
}

#[test]
fn ldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 20, 179], OperandSize::Qword)
}

