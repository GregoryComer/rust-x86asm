use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 203], OperandSize::Word)
}

#[test]
fn cmpxchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(BP, 134, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 158, 134, 0], OperandSize::Word)
}

#[test]
fn cmpxchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 210], OperandSize::Dword)
}

#[test]
fn cmpxchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 12, 147], OperandSize::Dword)
}

#[test]
fn cmpxchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 201], OperandSize::Qword)
}

#[test]
fn cmpxchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 20, 155], OperandSize::Qword)
}

#[test]
fn cmpxchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 209], OperandSize::Qword)
}

#[test]
fn cmpxchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 12, 203], OperandSize::Qword)
}

#[test]
fn cmpxchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 227], OperandSize::Word)
}

#[test]
fn cmpxchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 82, 29], OperandSize::Word)
}

#[test]
fn cmpxchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 204], OperandSize::Dword)
}

#[test]
fn cmpxchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(ECX, 1989101242, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 161, 186, 70, 143, 118], OperandSize::Dword)
}

#[test]
fn cmpxchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 251], OperandSize::Qword)
}

#[test]
fn cmpxchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 52, 70], OperandSize::Qword)
}

#[test]
fn cmpxchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 212], OperandSize::Word)
}

#[test]
fn cmpxchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 56], OperandSize::Word)
}

#[test]
fn cmpxchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 249], OperandSize::Dword)
}

#[test]
fn cmpxchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 15], OperandSize::Dword)
}

#[test]
fn cmpxchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 201], OperandSize::Qword)
}

#[test]
fn cmpxchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1361042248, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 180, 129, 72, 219, 31, 81], OperandSize::Qword)
}

#[test]
fn cmpxchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 241], OperandSize::Qword)
}

#[test]
fn cmpxchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(RSI, Two, 191725300, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 12, 117, 244, 126, 109, 11], OperandSize::Qword)
}

