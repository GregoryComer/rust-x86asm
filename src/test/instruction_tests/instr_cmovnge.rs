use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 201], OperandSize::Word)
}

#[test]
fn cmovnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 1882, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 147, 90, 7], OperandSize::Word)
}

#[test]
fn cmovnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 228], OperandSize::Dword)
}

#[test]
fn cmovnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 52, 251], OperandSize::Dword)
}

#[test]
fn cmovnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 201], OperandSize::Qword)
}

#[test]
fn cmovnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1788489429, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 60, 213, 213, 46, 154, 106], OperandSize::Qword)
}

#[test]
fn cmovnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 202], OperandSize::Word)
}

#[test]
fn cmovnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 36], OperandSize::Word)
}

#[test]
fn cmovnge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 241], OperandSize::Dword)
}

#[test]
fn cmovnge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EBX, 394003230, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 155, 30, 3, 124, 23], OperandSize::Dword)
}

#[test]
fn cmovnge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 220], OperandSize::Qword)
}

#[test]
fn cmovnge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1389181449, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 36, 85, 9, 58, 205, 82], OperandSize::Qword)
}

#[test]
fn cmovnge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 253], OperandSize::Qword)
}

#[test]
fn cmovnge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RBX, 248297857, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 147, 129, 185, 204, 14], OperandSize::Qword)
}

