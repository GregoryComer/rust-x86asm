use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(BP, 22769, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(BP, 14190, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Word)
}

#[test]
fn cmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(EDX, 1885590649, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Dword)
}

#[test]
fn cmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Byte), None)), operand2: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Qword)
}

#[test]
fn cmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 195, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(BX, 340, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Word)
}

#[test]
fn cmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Dword)
}

#[test]
fn cmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(RSI, 2122879665, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1852663763, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Qword)
}

#[test]
fn cmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(BX, 21809, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(SI, 223, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Word)
}

#[test]
fn cmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(EDI, 668671566, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Dword)
}

#[test]
fn cmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1153542928, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 881447016, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Qword)
}

#[test]
fn cmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1837828888, Some(OperandSize::Qword), None)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 574701902, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 167], OperandSize::Qword)
}

