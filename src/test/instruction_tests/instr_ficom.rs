use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 19], OperandSize::Word)
}

#[test]
fn ficom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 685852312, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 148, 130, 152, 70, 225, 40], OperandSize::Dword)
}

#[test]
fn ficom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 18], OperandSize::Qword)
}

#[test]
fn ficom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 3852, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 147, 12, 15], OperandSize::Word)
}

#[test]
fn ficom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 22], OperandSize::Dword)
}

#[test]
fn ficom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 19], OperandSize::Qword)
}

