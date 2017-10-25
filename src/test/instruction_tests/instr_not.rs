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
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 19], OperandSize::Word)
}

#[test]
fn not_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 211], OperandSize::Dword)
}

#[test]
fn not_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(EBX, Two, 141896319, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 93, 127, 42, 117, 8], OperandSize::Dword)
}

#[test]
fn not_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Qword)
}

#[test]
fn not_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 22], OperandSize::Qword)
}

#[test]
fn not_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Qword)
}

#[test]
fn not_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 20, 73], OperandSize::Qword)
}

#[test]
fn not_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 213], OperandSize::Word)
}

#[test]
fn not_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(BX, 9306, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 151, 90, 36], OperandSize::Word)
}

#[test]
fn not_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 211], OperandSize::Dword)
}

#[test]
fn not_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1886294082, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 20, 133, 66, 144, 110, 112], OperandSize::Dword)
}

#[test]
fn not_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 211], OperandSize::Qword)
}

#[test]
fn not_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RDX, 719391064, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 146, 88, 9, 225, 42], OperandSize::Qword)
}

#[test]
fn not_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 211], OperandSize::Word)
}

#[test]
fn not_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 23], OperandSize::Word)
}

#[test]
fn not_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 215], OperandSize::Dword)
}

#[test]
fn not_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(EBX, 845993239, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 147, 23, 213, 108, 50], OperandSize::Dword)
}

#[test]
fn not_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 215], OperandSize::Qword)
}

#[test]
fn not_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RCX, 503395708, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 145, 124, 53, 1, 30], OperandSize::Qword)
}

#[test]
fn not_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 211], OperandSize::Qword)
}

#[test]
fn not_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 18], OperandSize::Qword)
}

