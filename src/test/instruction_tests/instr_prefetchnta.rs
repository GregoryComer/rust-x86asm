use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchnta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 3], OperandSize::Word)
}

#[test]
fn prefetchnta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 827871510, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 4, 213, 22, 81, 88, 49], OperandSize::Dword)
}

#[test]
fn prefetchnta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 864473792, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 132, 83, 192, 210, 134, 51], OperandSize::Qword)
}

