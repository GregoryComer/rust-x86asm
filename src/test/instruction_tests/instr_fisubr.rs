use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(Memory(1885, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 46, 93, 7], OperandSize::Word)
}

#[test]
fn fisubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 44, 65], OperandSize::Dword)
}

#[test]
fn fisubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 44, 178], OperandSize::Qword)
}

#[test]
fn fisubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 43], OperandSize::Word)
}

#[test]
fn fisubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(EAX, 1076448446, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 168, 190, 76, 41, 64], OperandSize::Dword)
}

#[test]
fn fisubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 47], OperandSize::Qword)
}

