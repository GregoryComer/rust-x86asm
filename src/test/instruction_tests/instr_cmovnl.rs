use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 249], OperandSize::Word)
}

#[test]
fn cmovnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BP)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 45], OperandSize::Word)
}

#[test]
fn cmovnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 203], OperandSize::Dword)
}

#[test]
fn cmovnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 36, 191], OperandSize::Dword)
}

#[test]
fn cmovnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 205], OperandSize::Qword)
}

#[test]
fn cmovnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 52, 201], OperandSize::Qword)
}

#[test]
fn cmovnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 205], OperandSize::Word)
}

#[test]
fn cmovnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 16, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 83, 16], OperandSize::Word)
}

#[test]
fn cmovnl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 236], OperandSize::Dword)
}

#[test]
fn cmovnl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2092239734, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 28, 205, 118, 11, 181, 124], OperandSize::Dword)
}

#[test]
fn cmovnl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 218], OperandSize::Qword)
}

#[test]
fn cmovnl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 36, 80], OperandSize::Qword)
}

#[test]
fn cmovnl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 209], OperandSize::Qword)
}

#[test]
fn cmovnl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RCX, 643396970, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 169, 106, 117, 89, 38], OperandSize::Qword)
}

