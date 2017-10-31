use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnz_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 205], OperandSize::Word)
}

#[test]
fn cmovnz_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 56], OperandSize::Word)
}

#[test]
fn cmovnz_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 219], OperandSize::Dword)
}

#[test]
fn cmovnz_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 36, 82], OperandSize::Dword)
}

#[test]
fn cmovnz_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 211], OperandSize::Qword)
}

#[test]
fn cmovnz_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(CX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 9], OperandSize::Qword)
}

#[test]
fn cmovnz_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 227], OperandSize::Word)
}

#[test]
fn cmovnz_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 94, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 121, 94], OperandSize::Word)
}

#[test]
fn cmovnz_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 206], OperandSize::Dword)
}

#[test]
fn cmovnz_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 2124743767, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 60, 77, 87, 4, 165, 126], OperandSize::Dword)
}

#[test]
fn cmovnz_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 243], OperandSize::Qword)
}

#[test]
fn cmovnz_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RDI, 1129882159, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 175, 47, 162, 88, 67], OperandSize::Qword)
}

#[test]
fn cmovnz_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 238], OperandSize::Qword)
}

#[test]
fn cmovnz_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 32], OperandSize::Qword)
}

