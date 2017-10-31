use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 218], OperandSize::Word)
}

#[test]
fn cmpxchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 25], OperandSize::Word)
}

#[test]
fn cmpxchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 209], OperandSize::Dword)
}

#[test]
fn cmpxchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 575317251, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 140, 90, 3, 165, 74, 34], OperandSize::Dword)
}

#[test]
fn cmpxchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 209], OperandSize::Qword)
}

#[test]
fn cmpxchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(RDX, 864152886, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 146, 54, 237, 129, 51], OperandSize::Qword)
}

#[test]
fn cmpxchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 217], OperandSize::Qword)
}

#[test]
fn cmpxchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 28, 67], OperandSize::Qword)
}

#[test]
fn cmpxchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 233], OperandSize::Word)
}

#[test]
fn cmpxchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(DI, 285, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 189, 29, 1], OperandSize::Word)
}

#[test]
fn cmpxchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 235], OperandSize::Dword)
}

#[test]
fn cmpxchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1665717730, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 52, 253, 226, 213, 72, 99], OperandSize::Dword)
}

#[test]
fn cmpxchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 202], OperandSize::Qword)
}

#[test]
fn cmpxchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 15], OperandSize::Qword)
}

#[test]
fn cmpxchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 215], OperandSize::Word)
}

#[test]
fn cmpxchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(BX, 14305, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 167, 225, 55], OperandSize::Word)
}

#[test]
fn cmpxchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 254], OperandSize::Dword)
}

#[test]
fn cmpxchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 476277411, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 52, 221, 163, 106, 99, 28], OperandSize::Dword)
}

#[test]
fn cmpxchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 254], OperandSize::Qword)
}

#[test]
fn cmpxchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 44, 87], OperandSize::Qword)
}

#[test]
fn cmpxchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 214], OperandSize::Qword)
}

#[test]
fn cmpxchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 52, 223], OperandSize::Qword)
}

