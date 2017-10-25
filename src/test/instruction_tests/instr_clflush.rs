use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflush_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 61], OperandSize::Word)
}

#[test]
fn clflush_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectDisplaced(EDI, 911988131, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 191, 163, 213, 91, 54], OperandSize::Dword)
}

#[test]
fn clflush_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2123492093, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 60, 149, 253, 234, 145, 126], OperandSize::Qword)
}

