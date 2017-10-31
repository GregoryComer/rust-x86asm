use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(BP, 172, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 174, 172, 0], OperandSize::Word)
}

#[test]
fn fisubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1861555989, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 71, 21, 23, 245, 110], OperandSize::Dword)
}

#[test]
fn fisubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1495452118, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 86, 214, 201, 34, 89], OperandSize::Qword)
}

#[test]
fn fisubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 31050, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 171, 74, 121], OperandSize::Word)
}

#[test]
fn fisubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1661375939, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 172, 190, 195, 149, 6, 99], OperandSize::Dword)
}

#[test]
fn fisubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1370016420, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 172, 187, 164, 202, 168, 81], OperandSize::Qword)
}

