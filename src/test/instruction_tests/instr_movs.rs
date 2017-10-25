use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(BP, 945, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Word)
}

#[test]
fn movs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 187818929, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Dword)
}

#[test]
fn movs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(RAX, 2105829654, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Qword)
}

#[test]
fn movs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 30146, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Word)
}

#[test]
fn movs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 901516033, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Dword)
}

#[test]
fn movs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Qword)
}

#[test]
fn movs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Memory(13738, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Word)
}

#[test]
fn movs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Dword)
}

#[test]
fn movs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1471830648, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Qword)
}

#[test]
fn movs_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(RSI, Two, 662486415, Some(OperandSize::Qword), None)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 165], OperandSize::Qword)
}

