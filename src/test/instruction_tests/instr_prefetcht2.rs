use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectDisplaced(DI, 25418, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 157, 74, 99], OperandSize::Word)
}

#[test]
fn prefetcht2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectDisplaced(ECX, 819113232, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 153, 16, 173, 210, 48], OperandSize::Dword)
}

#[test]
fn prefetcht2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 25], OperandSize::Qword)
}

