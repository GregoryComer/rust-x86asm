use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnz_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 255], OperandSize::Word)
}

#[test]
fn cmovnz_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 16847, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 160, 207, 65], OperandSize::Word)
}

#[test]
fn cmovnz_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 226], OperandSize::Dword)
}

#[test]
fn cmovnz_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 20, 155], OperandSize::Dword)
}

#[test]
fn cmovnz_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 253], OperandSize::Qword)
}

#[test]
fn cmovnz_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RCX, 1330508524, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 137, 236, 242, 77, 79], OperandSize::Qword)
}

#[test]
fn cmovnz_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 218], OperandSize::Word)
}

#[test]
fn cmovnz_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 57], OperandSize::Word)
}

#[test]
fn cmovnz_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 221], OperandSize::Dword)
}

#[test]
fn cmovnz_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 39], OperandSize::Dword)
}

#[test]
fn cmovnz_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 243], OperandSize::Qword)
}

#[test]
fn cmovnz_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 52, 199], OperandSize::Qword)
}

#[test]
fn cmovnz_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 227], OperandSize::Qword)
}

#[test]
fn cmovnz_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 60, 145], OperandSize::Qword)
}

