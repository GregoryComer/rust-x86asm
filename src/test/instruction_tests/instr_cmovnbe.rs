use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 252], OperandSize::Word)
}

#[test]
fn cmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(DI, 221, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 165, 221, 0], OperandSize::Word)
}

#[test]
fn cmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 252], OperandSize::Dword)
}

#[test]
fn cmovnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BX)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 27], OperandSize::Dword)
}

#[test]
fn cmovnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 230], OperandSize::Qword)
}

#[test]
fn cmovnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1034970849, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 148, 192, 225, 102, 176, 61], OperandSize::Qword)
}

#[test]
fn cmovnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 203], OperandSize::Word)
}

#[test]
fn cmovnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 56], OperandSize::Word)
}

#[test]
fn cmovnbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 252], OperandSize::Dword)
}

#[test]
fn cmovnbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 60, 177], OperandSize::Dword)
}

#[test]
fn cmovnbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 202], OperandSize::Qword)
}

#[test]
fn cmovnbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RDX, 708397463, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 186, 151, 73, 57, 42], OperandSize::Qword)
}

#[test]
fn cmovnbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 210], OperandSize::Qword)
}

#[test]
fn cmovnbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1474993915, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 156, 134, 251, 158, 234, 87], OperandSize::Qword)
}

