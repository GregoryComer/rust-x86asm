use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovpe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 254], OperandSize::Word)
}

#[test]
fn cmovpe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 4060, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 137, 220, 15], OperandSize::Word)
}

#[test]
fn cmovpe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 201], OperandSize::Dword)
}

#[test]
fn cmovpe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 447202594, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 20, 197, 34, 197, 167, 26], OperandSize::Dword)
}

#[test]
fn cmovpe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 225], OperandSize::Qword)
}

#[test]
fn cmovpe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RSI, 959843363, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 182, 35, 12, 54, 57], OperandSize::Qword)
}

#[test]
fn cmovpe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 217], OperandSize::Word)
}

#[test]
fn cmovpe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 146, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 155, 146, 0], OperandSize::Word)
}

#[test]
fn cmovpe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 212], OperandSize::Dword)
}

#[test]
fn cmovpe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 19], OperandSize::Dword)
}

#[test]
fn cmovpe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 229], OperandSize::Qword)
}

#[test]
fn cmovpe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 20, 128], OperandSize::Qword)
}

#[test]
fn cmovpe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 210], OperandSize::Qword)
}

#[test]
fn cmovpe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1116284696, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 44, 85, 24, 39, 137, 66], OperandSize::Qword)
}

