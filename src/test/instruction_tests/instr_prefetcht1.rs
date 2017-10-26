use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 16], OperandSize::Word)
}

#[test]
fn prefetcht1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 804818033, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 148, 176, 113, 140, 248, 47], OperandSize::Dword)
}

#[test]
fn prefetcht1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 20, 193], OperandSize::Qword)
}

