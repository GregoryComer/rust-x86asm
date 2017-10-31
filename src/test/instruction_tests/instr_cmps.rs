use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 11557, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Word)
}

#[test]
fn cmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 555232574, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Dword)
}

#[test]
fn cmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 290165885, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Qword)
}

#[test]
fn cmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Word)
}

#[test]
fn cmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1589153546, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Dword)
}

#[test]
fn cmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1314837901, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Qword)
}

#[test]
fn cmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(DI, 102, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 230, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Word)
}

#[test]
fn cmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1047163800, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Dword)
}

#[test]
fn cmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(RBX, Two, 574426495, Some(OperandSize::Dword), None)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Qword)
}

#[test]
fn cmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1829262538, Some(OperandSize::Qword), None)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 167], OperandSize::Qword)
}

