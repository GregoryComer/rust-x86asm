use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stos_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 203, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Word)
}

#[test]
fn stos_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1152119332, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Dword)
}

#[test]
fn stos_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 358669770, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Qword)
}

#[test]
fn stos_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Word)
}

#[test]
fn stos_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 2045177442, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Dword)
}

#[test]
fn stos_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(RBX, 664731441, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Qword)
}

#[test]
fn stos_7() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(DI, 18058, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Word)
}

#[test]
fn stos_8() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(ECX, 128031086, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Dword)
}

#[test]
fn stos_9() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Qword)
}

#[test]
fn stos_10() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 171], OperandSize::Qword)
}

