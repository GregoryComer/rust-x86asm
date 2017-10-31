use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectDisplaced(EBX, 1596030728, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 147, 8, 127, 33, 95], OperandSize::Dword)
}

#[test]
fn vldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 20, 198], OperandSize::Qword)
}

