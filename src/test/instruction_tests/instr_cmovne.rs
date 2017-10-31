use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 239], OperandSize::Word)
}

#[test]
fn cmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(BX, 177, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 143, 177, 0], OperandSize::Word)
}

#[test]
fn cmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 223], OperandSize::Dword)
}

#[test]
fn cmovne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 392554011, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 164, 158, 27, 230, 101, 23], OperandSize::Dword)
}

#[test]
fn cmovne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 242], OperandSize::Qword)
}

#[test]
fn cmovne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 24], OperandSize::Qword)
}

#[test]
fn cmovne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 201], OperandSize::Word)
}

#[test]
fn cmovne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27337, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 161, 201, 106], OperandSize::Word)
}

#[test]
fn cmovne_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 245], OperandSize::Dword)
}

#[test]
fn cmovne_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 39], OperandSize::Dword)
}

#[test]
fn cmovne_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 202], OperandSize::Qword)
}

#[test]
fn cmovne_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 60, 130], OperandSize::Qword)
}

#[test]
fn cmovne_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 253], OperandSize::Qword)
}

#[test]
fn cmovne_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 545105865, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 140, 82, 201, 167, 125, 32], OperandSize::Qword)
}

