use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 244], OperandSize::Word)
}

#[test]
fn cmovge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BX, 10305, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 175, 65, 40], OperandSize::Word)
}

#[test]
fn cmovge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 212], OperandSize::Dword)
}

#[test]
fn cmovge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 28, 70], OperandSize::Dword)
}

#[test]
fn cmovge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 238], OperandSize::Qword)
}

#[test]
fn cmovge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RCX, 1021001240, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 137, 24, 62, 219, 60], OperandSize::Qword)
}

#[test]
fn cmovge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 206], OperandSize::Word)
}

#[test]
fn cmovge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 19307, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 161, 107, 75], OperandSize::Word)
}

#[test]
fn cmovge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 227], OperandSize::Dword)
}

#[test]
fn cmovge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1504278179, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 156, 187, 163, 118, 169, 89], OperandSize::Dword)
}

#[test]
fn cmovge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 231], OperandSize::Qword)
}

#[test]
fn cmovge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 43], OperandSize::Qword)
}

#[test]
fn cmovge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 203], OperandSize::Qword)
}

#[test]
fn cmovge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 52, 200], OperandSize::Qword)
}

