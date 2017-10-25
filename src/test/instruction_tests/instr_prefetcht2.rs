use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectDisplaced(BX, 80, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 95, 80], OperandSize::Word)
}

#[test]
fn prefetcht2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1936307071, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 156, 129, 127, 179, 105, 115], OperandSize::Dword)
}

#[test]
fn prefetcht2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1913314564, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 156, 73, 4, 221, 10, 114], OperandSize::Qword)
}

