use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn not_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Word)
}

#[test]
fn not_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(BP, 8232, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 150, 40, 32], OperandSize::Word)
}

#[test]
fn not_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Dword)
}

#[test]
fn not_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(EDX, 1752671118, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 146, 142, 163, 119, 104], OperandSize::Dword)
}

#[test]
fn not_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Qword)
}

#[test]
fn not_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1395112094, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 148, 194, 158, 184, 39, 83], OperandSize::Qword)
}

#[test]
fn not_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 211], OperandSize::Qword)
}

#[test]
fn not_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 219], OperandSize::Qword)
}

#[test]
fn not_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 211], OperandSize::Word)
}

#[test]
fn not_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 16], OperandSize::Word)
}

#[test]
fn not_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 209], OperandSize::Dword)
}

#[test]
fn not_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(EDX, 936345343, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 146, 255, 126, 207, 55], OperandSize::Dword)
}

#[test]
fn not_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 210], OperandSize::Qword)
}

#[test]
fn not_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2144162343, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 20, 149, 39, 82, 205, 127], OperandSize::Qword)
}

#[test]
fn not_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 214], OperandSize::Word)
}

#[test]
fn not_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 19], OperandSize::Word)
}

#[test]
fn not_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 214], OperandSize::Dword)
}

#[test]
fn not_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 20, 187], OperandSize::Dword)
}

#[test]
fn not_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 209], OperandSize::Qword)
}

#[test]
fn not_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 22], OperandSize::Qword)
}

#[test]
fn not_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 210], OperandSize::Qword)
}

#[test]
fn not_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1665159814, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 20, 245, 134, 82, 64, 99], OperandSize::Qword)
}

