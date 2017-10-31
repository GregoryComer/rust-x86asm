use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 12, 202], OperandSize::Dword)
}

#[test]
fn prefetchw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 461772944, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 140, 136, 144, 24, 134, 27], OperandSize::Qword)
}

