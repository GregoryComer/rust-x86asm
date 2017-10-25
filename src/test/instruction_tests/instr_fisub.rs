use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(BP, 23306, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 166, 10, 91], OperandSize::Word)
}

#[test]
fn fisub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(ECX, 1402266064, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 161, 208, 225, 148, 83], OperandSize::Dword)
}

#[test]
fn fisub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 36, 134], OperandSize::Qword)
}

#[test]
fn fisub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 37], OperandSize::Word)
}

#[test]
fn fisub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 74], OperandSize::Dword)
}

#[test]
fn fisub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1532730879, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 181, 255, 157, 91, 91], OperandSize::Qword)
}

