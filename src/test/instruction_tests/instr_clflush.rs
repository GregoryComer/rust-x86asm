use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflush_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 186, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 184, 186, 0], OperandSize::Word)
}

#[test]
fn clflush_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledDisplaced(ECX, Four, 594114510, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 60, 141, 206, 119, 105, 35], OperandSize::Dword)
}

#[test]
fn clflush_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectDisplaced(RDI, 213969566, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 191, 158, 234, 192, 12], OperandSize::Qword)
}

