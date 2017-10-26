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
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 702156021, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Dword)
}

#[test]
fn lods_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Qword)
}

#[test]
fn lods_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Word)
}

#[test]
fn lods_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Dword)
}

#[test]
fn lods_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 166126744, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Qword)
}

#[test]
fn lods_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 11867, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Word)
}

#[test]
fn lods_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 531408245, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Dword)
}

#[test]
fn lods_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RCX, Four, 607244171, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Qword)
}

#[test]
fn lods_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RSI, Two, 879147376, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 173], OperandSize::Qword)
}

