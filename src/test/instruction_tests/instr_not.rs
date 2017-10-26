use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn not_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Word)
}

#[test]
fn not_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 31115, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 144, 139, 121], OperandSize::Word)
}

#[test]
fn not_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Dword)
}

#[test]
fn not_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 2032122076, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 148, 214, 220, 184, 31, 121], OperandSize::Dword)
}

#[test]
fn not_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Qword)
}

#[test]
fn not_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RDI, 868710222, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 151, 78, 119, 199, 51], OperandSize::Qword)
}

#[test]
fn not_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Qword)
}

#[test]
fn not_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1338946581, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 93, 21, 180, 206, 79], OperandSize::Qword)
}

#[test]
fn not_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 211], OperandSize::Word)
}

#[test]
fn not_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 17], OperandSize::Word)
}

#[test]
fn not_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 209], OperandSize::Dword)
}

#[test]
fn not_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(ESI, 430992642, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 150, 2, 109, 176, 25], OperandSize::Dword)
}

#[test]
fn not_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 211], OperandSize::Qword)
}

#[test]
fn not_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 17], OperandSize::Qword)
}

#[test]
fn not_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 209], OperandSize::Word)
}

#[test]
fn not_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 16], OperandSize::Word)
}

#[test]
fn not_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 209], OperandSize::Dword)
}

#[test]
fn not_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(ECX, 1777013928, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 145, 168, 20, 235, 105], OperandSize::Dword)
}

#[test]
fn not_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 213], OperandSize::Qword)
}

#[test]
fn not_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1422405252, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 20, 181, 132, 46, 200, 84], OperandSize::Qword)
}

#[test]
fn not_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 211], OperandSize::Qword)
}

#[test]
fn not_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 20, 193], OperandSize::Qword)
}

