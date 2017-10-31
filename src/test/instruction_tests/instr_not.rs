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
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 16], OperandSize::Word)
}

#[test]
fn not_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 211], OperandSize::Dword)
}

#[test]
fn not_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 91], OperandSize::Dword)
}

#[test]
fn not_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Qword)
}

#[test]
fn not_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RCX, Four, 2120121958, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 141, 102, 126, 94, 126], OperandSize::Qword)
}

#[test]
fn not_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Qword)
}

#[test]
fn not_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1011053698, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 148, 187, 130, 116, 67, 60], OperandSize::Qword)
}

#[test]
fn not_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 209], OperandSize::Word)
}

#[test]
fn not_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(BX, 22497, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 151, 225, 87], OperandSize::Word)
}

#[test]
fn not_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 215], OperandSize::Dword)
}

#[test]
fn not_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 16], OperandSize::Dword)
}

#[test]
fn not_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 211], OperandSize::Qword)
}

#[test]
fn not_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 19], OperandSize::Qword)
}

#[test]
fn not_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 214], OperandSize::Word)
}

#[test]
fn not_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 210, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 146, 210, 0], OperandSize::Word)
}

#[test]
fn not_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 212], OperandSize::Dword)
}

#[test]
fn not_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(EDX, Four, 489486758, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 20, 149, 166, 249, 44, 29], OperandSize::Dword)
}

#[test]
fn not_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 214], OperandSize::Qword)
}

#[test]
fn not_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 18], OperandSize::Qword)
}

#[test]
fn not_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 209], OperandSize::Qword)
}

#[test]
fn not_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1829681526, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 20, 77, 118, 185, 14, 109], OperandSize::Qword)
}

