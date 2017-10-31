use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 219], OperandSize::Word)
}

#[test]
fn cmpxchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 29], OperandSize::Word)
}

#[test]
fn cmpxchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 202], OperandSize::Dword)
}

#[test]
fn cmpxchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(EBX, 46899939, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 139, 227, 162, 203, 2], OperandSize::Dword)
}

#[test]
fn cmpxchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 209], OperandSize::Qword)
}

#[test]
fn cmpxchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(RDI, 1857053682, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 143, 242, 99, 176, 110], OperandSize::Qword)
}

#[test]
fn cmpxchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 211], OperandSize::Qword)
}

#[test]
fn cmpxchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(RDX, 374998646, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 154, 118, 6, 90, 22], OperandSize::Qword)
}

#[test]
fn cmpxchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 205], OperandSize::Word)
}

#[test]
fn cmpxchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 41], OperandSize::Word)
}

#[test]
fn cmpxchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 241], OperandSize::Dword)
}

#[test]
fn cmpxchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1654296501, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 52, 221, 181, 143, 154, 98], OperandSize::Dword)
}

#[test]
fn cmpxchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 247], OperandSize::Qword)
}

#[test]
fn cmpxchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 41], OperandSize::Qword)
}

#[test]
fn cmpxchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 227], OperandSize::Word)
}

#[test]
fn cmpxchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 24], OperandSize::Word)
}

#[test]
fn cmpxchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 219], OperandSize::Dword)
}

#[test]
fn cmpxchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 44, 211], OperandSize::Dword)
}

#[test]
fn cmpxchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 253], OperandSize::Qword)
}

#[test]
fn cmpxchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 12986685, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 156, 131, 61, 41, 198, 0], OperandSize::Qword)
}

#[test]
fn cmpxchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 239], OperandSize::Qword)
}

#[test]
fn cmpxchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 33], OperandSize::Qword)
}

