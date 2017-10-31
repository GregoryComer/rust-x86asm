use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 253], OperandSize::Word)
}

#[test]
fn cmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 182, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 168, 182, 0], OperandSize::Word)
}

#[test]
fn cmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 215], OperandSize::Dword)
}

#[test]
fn cmovbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 733317847, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 188, 79, 215, 138, 181, 43], OperandSize::Dword)
}

#[test]
fn cmovbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 206], OperandSize::Qword)
}

#[test]
fn cmovbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SI)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 55], OperandSize::Qword)
}

#[test]
fn cmovbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 209], OperandSize::Word)
}

#[test]
fn cmovbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 23114, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 184, 74, 90], OperandSize::Word)
}

#[test]
fn cmovbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 255], OperandSize::Dword)
}

#[test]
fn cmovbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EBX, 1026116233, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 179, 137, 74, 41, 61], OperandSize::Dword)
}

#[test]
fn cmovbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 235], OperandSize::Qword)
}

#[test]
fn cmovbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1074975175, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 188, 146, 199, 209, 18, 64], OperandSize::Qword)
}

#[test]
fn cmovbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 212], OperandSize::Qword)
}

#[test]
fn cmovbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 52, 147], OperandSize::Qword)
}

