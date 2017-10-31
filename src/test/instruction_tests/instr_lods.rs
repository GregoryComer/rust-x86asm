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
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Dword)
}

#[test]
fn lods_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1584360447, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Qword)
}

#[test]
fn lods_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 18563, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Word)
}

#[test]
fn lods_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(ESI, Four, 236130943, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Dword)
}

#[test]
fn lods_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1832552192, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Qword)
}

#[test]
fn lods_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(SI, 15330, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Word)
}

#[test]
fn lods_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1994474443, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Dword)
}

#[test]
fn lods_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(RDX, 653217826, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Qword)
}

#[test]
fn lods_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 173], OperandSize::Qword)
}

