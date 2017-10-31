use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 245], OperandSize::Word)
}

#[test]
fn cmovnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 202, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 177, 202, 0], OperandSize::Word)
}

#[test]
fn cmovnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 234], OperandSize::Dword)
}

#[test]
fn cmovnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 9], OperandSize::Dword)
}

#[test]
fn cmovnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 255], OperandSize::Qword)
}

#[test]
fn cmovnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RCX, 1253553567, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 169, 159, 181, 183, 74], OperandSize::Qword)
}

#[test]
fn cmovnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 212], OperandSize::Word)
}

#[test]
fn cmovnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 126, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 105, 126], OperandSize::Word)
}

#[test]
fn cmovnl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 230], OperandSize::Dword)
}

#[test]
fn cmovnl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 24], OperandSize::Dword)
}

#[test]
fn cmovnl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 236], OperandSize::Qword)
}

#[test]
fn cmovnl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RCX, 1736755848, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 177, 136, 202, 132, 103], OperandSize::Qword)
}

#[test]
fn cmovnl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 235], OperandSize::Qword)
}

#[test]
fn cmovnl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 2085107261, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 20, 125, 61, 54, 72, 124], OperandSize::Qword)
}

