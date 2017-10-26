use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stos_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Memory(29855, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Word)
}

#[test]
fn stos_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Dword)
}

#[test]
fn stos_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1583074741, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Qword)
}

#[test]
fn stos_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 40, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Word)
}

#[test]
fn stos_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1172970744, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Dword)
}

#[test]
fn stos_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Qword)
}

#[test]
fn stos_7() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Word)
}

#[test]
fn stos_8() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 228341196, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Dword)
}

#[test]
fn stos_9() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Qword)
}

#[test]
fn stos_10() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 171], OperandSize::Qword)
}

