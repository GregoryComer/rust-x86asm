use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 214], OperandSize::Word)
}

#[test]
fn cmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 29404, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 140, 220, 114], OperandSize::Word)
}

#[test]
fn cmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 235], OperandSize::Dword)
}

#[test]
fn cmovnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BP)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 43], OperandSize::Dword)
}

#[test]
fn cmovnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 231], OperandSize::Qword)
}

#[test]
fn cmovnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 12, 211], OperandSize::Qword)
}

#[test]
fn cmovnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 234], OperandSize::Word)
}

#[test]
fn cmovnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 23243, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 170, 203, 90], OperandSize::Word)
}

#[test]
fn cmovnb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 244], OperandSize::Dword)
}

#[test]
fn cmovnb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 22], OperandSize::Dword)
}

#[test]
fn cmovnb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 210], OperandSize::Qword)
}

#[test]
fn cmovnb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 60, 200], OperandSize::Qword)
}

#[test]
fn cmovnb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 209], OperandSize::Qword)
}

#[test]
fn cmovnb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1785183842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 172, 123, 98, 190, 103, 106], OperandSize::Qword)
}

