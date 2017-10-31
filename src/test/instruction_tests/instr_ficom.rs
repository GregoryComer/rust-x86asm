use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25230, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 147, 142, 98], OperandSize::Word)
}

#[test]
fn ficom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 20, 176], OperandSize::Dword)
}

#[test]
fn ficom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 16], OperandSize::Qword)
}

#[test]
fn ficom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 77, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 82, 77], OperandSize::Word)
}

#[test]
fn ficom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 22], OperandSize::Dword)
}

#[test]
fn ficom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 22], OperandSize::Qword)
}

