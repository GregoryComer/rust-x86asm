use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lods_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Word)
}

#[test]
fn lods_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(EAX, 1692799751, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Dword)
}

#[test]
fn lods_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Qword)
}

#[test]
fn lods_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 26131, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Word)
}

#[test]
fn lods_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Dword)
}

#[test]
fn lods_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Qword)
}

#[test]
fn lods_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Word)
}

#[test]
fn lods_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Dword)
}

#[test]
fn lods_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RCX, Two, 552269346, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Qword)
}

#[test]
fn lods_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 173], OperandSize::Qword)
}

