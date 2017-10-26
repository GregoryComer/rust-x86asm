use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 60, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 99, 60], OperandSize::Word)
}

#[test]
fn fbld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1908781746, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 36, 213, 178, 178, 197, 113], OperandSize::Dword)
}

#[test]
fn fbld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 36, 199], OperandSize::Qword)
}

