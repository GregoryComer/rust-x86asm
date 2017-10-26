use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchnta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(Memory(13148, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 6, 92, 51], OperandSize::Word)
}

#[test]
fn prefetchnta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1336543540, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 4, 77, 52, 9, 170, 79], OperandSize::Dword)
}

#[test]
fn prefetchnta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 2], OperandSize::Qword)
}

