use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1671019788, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 148, 242, 12, 189, 153, 99], OperandSize::Dword)
}

#[test]
fn vldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 22], OperandSize::Qword)
}

