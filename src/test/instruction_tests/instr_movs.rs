use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(BX, 178, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(SI, 40, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Word)
}

#[test]
fn movs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(EBX, 699320489, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1485155612, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Dword)
}

#[test]
fn movs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(RCX, Two, 654088774, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(RAX, 906759383, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Qword)
}

#[test]
fn movs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 150, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 28, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Word)
}

#[test]
fn movs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1867163397, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Dword)
}

#[test]
fn movs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1035865769, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 326284557, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Qword)
}

#[test]
fn movs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Word)
}

#[test]
fn movs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(EAX, 638321349, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Dword)
}

#[test]
fn movs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1605323554, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Qword)
}

#[test]
fn movs_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(RSI, 686347013, Some(OperandSize::Qword), None)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 165], OperandSize::Qword)
}

