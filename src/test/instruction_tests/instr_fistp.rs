use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fistp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 31], OperandSize::Word)
}

#[test]
fn fistp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 813371761, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 156, 190, 113, 17, 123, 48], OperandSize::Dword)
}

#[test]
fn fistp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RBX, 423229486, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 155, 46, 248, 57, 25], OperandSize::Qword)
}

#[test]
fn fistp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 25], OperandSize::Word)
}

#[test]
fn fistp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 25], OperandSize::Dword)
}

#[test]
fn fistp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1529043099, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 28, 213, 155, 88, 35, 91], OperandSize::Qword)
}

#[test]
fn fistp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 82, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 121, 82], OperandSize::Word)
}

#[test]
fn fistp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1121262484, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 188, 126, 148, 27, 213, 66], OperandSize::Dword)
}

#[test]
fn fistp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1985455968, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 60, 93, 96, 167, 87, 118], OperandSize::Qword)
}

