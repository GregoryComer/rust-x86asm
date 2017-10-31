use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Memory(2121, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(SI, 9754, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Word)
}

#[test]
fn movs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(ECX, 545155892, Some(OperandSize::Byte), None)), operand2: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Dword)
}

#[test]
fn movs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(RAX, 234480979, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Qword)
}

#[test]
fn movs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13045, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 82, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Word)
}

#[test]
fn movs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(EDX, 465137710, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Dword)
}

#[test]
fn movs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Qword)
}

#[test]
fn movs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(BX, 218, Some(OperandSize::Dword), None)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Word)
}

#[test]
fn movs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(ECX, 2069825240, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(EBX, 731179226, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Dword)
}

#[test]
fn movs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(RDX, 70073651, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(RCX, 27589160, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Qword)
}

#[test]
fn movs_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(IndirectDisplaced(RDI, 1423924706, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 165], OperandSize::Qword)
}

