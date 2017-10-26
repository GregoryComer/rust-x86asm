use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 11803, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 144, 27, 46], OperandSize::Word)
}

#[test]
fn ficom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1647191441, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 148, 240, 145, 37, 46, 98], OperandSize::Dword)
}

#[test]
fn ficom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(RDX, 676118376, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 146, 104, 191, 76, 40], OperandSize::Qword)
}

#[test]
fn ficom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 67, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 82, 67], OperandSize::Word)
}

#[test]
fn ficom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 20, 126], OperandSize::Dword)
}

#[test]
fn ficom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledDisplaced(RDX, Four, 90887446, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 20, 149, 22, 213, 106, 5], OperandSize::Qword)
}

