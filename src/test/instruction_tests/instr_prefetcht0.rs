use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht0_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13813, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 139, 245, 53], OperandSize::Word)
}

#[test]
fn prefetcht0_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectDisplaced(EBX, 1364530700, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 139, 12, 22, 85, 81], OperandSize::Dword)
}

#[test]
fn prefetcht0_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 14], OperandSize::Qword)
}

