use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 23], OperandSize::Word)
}

#[test]
fn ficom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledDisplaced(EDX, Four, 743787457, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 20, 149, 193, 75, 85, 44], OperandSize::Dword)
}

#[test]
fn ficom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 23], OperandSize::Qword)
}

#[test]
fn ficom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(DI, 17814, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 149, 150, 69], OperandSize::Word)
}

#[test]
fn ficom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(EAX, 833458172, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 144, 252, 143, 173, 49], OperandSize::Dword)
}

#[test]
fn ficom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 22], OperandSize::Qword)
}

