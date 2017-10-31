use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stos_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 16670, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Word)
}

#[test]
fn stos_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 592399209, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Dword)
}

#[test]
fn stos_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(RDX, 1400967640, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Qword)
}

#[test]
fn stos_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Word)
}

#[test]
fn stos_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 668918088, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Dword)
}

#[test]
fn stos_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 124159431, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Qword)
}

#[test]
fn stos_7() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Word)
}

#[test]
fn stos_8() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(ECX, Two, 891728697, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Dword)
}

#[test]
fn stos_9() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1441952826, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Qword)
}

#[test]
fn stos_10() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1253203118, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 171], OperandSize::Qword)
}

