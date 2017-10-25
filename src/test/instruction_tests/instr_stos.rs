use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stos_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Word)
}

#[test]
fn stos_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Dword)
}

#[test]
fn stos_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 830054389, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Qword)
}

#[test]
fn stos_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 11431, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Word)
}

#[test]
fn stos_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(EAX, 923431317, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Dword)
}

#[test]
fn stos_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 197591696, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Qword)
}

#[test]
fn stos_7() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(DI, 9823, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Word)
}

#[test]
fn stos_8() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(EDI, 1977086923, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Dword)
}

#[test]
fn stos_9() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Qword)
}

#[test]
fn stos_10() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 171], OperandSize::Qword)
}

