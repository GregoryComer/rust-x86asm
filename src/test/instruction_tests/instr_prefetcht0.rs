use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht0_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 11], OperandSize::Word)
}

#[test]
fn prefetcht0_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1613905998, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 189, 78, 64, 50, 96], OperandSize::Dword)
}

#[test]
fn prefetcht0_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 178], OperandSize::Qword)
}

