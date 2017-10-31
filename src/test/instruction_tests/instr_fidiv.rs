use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectDisplaced(DI, 147, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 181, 147, 0], OperandSize::Word)
}

#[test]
fn fidiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 48], OperandSize::Dword)
}

#[test]
fn fidiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 48], OperandSize::Qword)
}

#[test]
fn fidiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 51], OperandSize::Word)
}

#[test]
fn fidiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 88], OperandSize::Dword)
}

#[test]
fn fidiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectDisplaced(RSI, 234157319, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 182, 7, 245, 244, 13], OperandSize::Qword)
}

