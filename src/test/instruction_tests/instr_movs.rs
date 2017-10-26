use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Word)
}

#[test]
fn movs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1651366485, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Dword)
}

#[test]
fn movs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(RBX, 67597585, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1762329585, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Qword)
}

#[test]
fn movs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(SI, 17796, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(SI, 20, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Word)
}

#[test]
fn movs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(EDI, Four, 264434268, Some(OperandSize::Word), None)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Dword)
}

#[test]
fn movs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 251148047, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Qword)
}

#[test]
fn movs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Word)
}

#[test]
fn movs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 2126295384, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Dword)
}

#[test]
fn movs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 128306878, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1787052710, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Qword)
}

#[test]
fn movs_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 165], OperandSize::Qword)
}

