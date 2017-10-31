use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovpe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 225], OperandSize::Word)
}

#[test]
fn cmovpe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17160, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 169, 8, 67], OperandSize::Word)
}

#[test]
fn cmovpe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 254], OperandSize::Dword)
}

#[test]
fn cmovpe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1667987348, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 20, 197, 148, 119, 107, 99], OperandSize::Dword)
}

#[test]
fn cmovpe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 252], OperandSize::Qword)
}

#[test]
fn cmovpe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1818245150, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 60, 245, 30, 56, 96, 108], OperandSize::Qword)
}

#[test]
fn cmovpe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 230], OperandSize::Word)
}

#[test]
fn cmovpe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(SI, 14118, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 140, 38, 55], OperandSize::Word)
}

#[test]
fn cmovpe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 227], OperandSize::Dword)
}

#[test]
fn cmovpe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 40], OperandSize::Dword)
}

#[test]
fn cmovpe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 209], OperandSize::Qword)
}

#[test]
fn cmovpe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RDI, 169660727, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 167, 55, 209, 28, 10], OperandSize::Qword)
}

#[test]
fn cmovpe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 212], OperandSize::Qword)
}

#[test]
fn cmovpe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 627113717, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 188, 159, 245, 254, 96, 37], OperandSize::Qword)
}

