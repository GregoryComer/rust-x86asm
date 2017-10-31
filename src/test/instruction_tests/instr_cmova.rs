use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmova_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 202], OperandSize::Word)
}

#[test]
fn cmova_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 9], OperandSize::Word)
}

#[test]
fn cmova_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 226], OperandSize::Dword)
}

#[test]
fn cmova_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDI, 1973693202, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 151, 18, 43, 164, 117], OperandSize::Dword)
}

#[test]
fn cmova_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 255], OperandSize::Qword)
}

#[test]
fn cmova_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 34], OperandSize::Qword)
}

#[test]
fn cmova_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 255], OperandSize::Word)
}

#[test]
fn cmova_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBP)), operand2: Some(Memory(27736, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 46, 88, 108], OperandSize::Word)
}

#[test]
fn cmova_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 210], OperandSize::Dword)
}

#[test]
fn cmova_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 2125107642, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 156, 114, 186, 145, 170, 126], OperandSize::Dword)
}

#[test]
fn cmova_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 206], OperandSize::Qword)
}

#[test]
fn cmova_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 60, 120], OperandSize::Qword)
}

#[test]
fn cmova_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 212], OperandSize::Qword)
}

#[test]
fn cmova_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 58], OperandSize::Qword)
}

