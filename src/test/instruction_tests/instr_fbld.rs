use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(Memory(14584, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 38, 248, 56], OperandSize::Word)
}

#[test]
fn fbld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 36, 222], OperandSize::Dword)
}

#[test]
fn fbld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectDisplaced(RAX, 1303534718, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 160, 126, 92, 178, 77], OperandSize::Qword)
}

