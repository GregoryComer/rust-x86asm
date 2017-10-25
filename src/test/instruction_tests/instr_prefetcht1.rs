use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17069, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 145, 173, 66], OperandSize::Word)
}

#[test]
fn prefetcht1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectDisplaced(ECX, 512508295, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 145, 135, 65, 140, 30], OperandSize::Dword)
}

#[test]
fn prefetcht1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectDisplaced(RBX, 1763120666, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 147, 26, 22, 23, 105], OperandSize::Qword)
}

