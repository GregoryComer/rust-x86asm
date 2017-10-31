use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchnta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 112, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 67, 112], OperandSize::Word)
}

#[test]
fn prefetchnta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectDisplaced(ECX, 258144524, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 129, 12, 249, 98, 15], OperandSize::Dword)
}

#[test]
fn prefetchnta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 890223075, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 132, 131, 227, 185, 15, 53], OperandSize::Qword)
}

