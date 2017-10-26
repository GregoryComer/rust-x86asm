use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 210], OperandSize::Word)
}

#[test]
fn cmovnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(BP, 189, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 142, 189, 0], OperandSize::Word)
}

#[test]
fn cmovnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 246], OperandSize::Dword)
}

#[test]
fn cmovnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 55], OperandSize::Dword)
}

#[test]
fn cmovnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 235], OperandSize::Qword)
}

#[test]
fn cmovnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 17], OperandSize::Qword)
}

#[test]
fn cmovnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 226], OperandSize::Word)
}

#[test]
fn cmovnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 11], OperandSize::Word)
}

#[test]
fn cmovnge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 243], OperandSize::Dword)
}

#[test]
fn cmovnge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDI)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 57], OperandSize::Dword)
}

#[test]
fn cmovnge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 201], OperandSize::Qword)
}

#[test]
fn cmovnge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 36, 222], OperandSize::Qword)
}

#[test]
fn cmovnge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 234], OperandSize::Qword)
}

#[test]
fn cmovnge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 531914930, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 28, 133, 178, 96, 180, 31], OperandSize::Qword)
}

