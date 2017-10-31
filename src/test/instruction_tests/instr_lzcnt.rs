use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 244], OperandSize::Word)
}

#[test]
fn lzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 35], OperandSize::Word)
}

#[test]
fn lzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 243], OperandSize::Dword)
}

#[test]
fn lzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 643741499, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 36, 69, 59, 183, 94, 38], OperandSize::Dword)
}

#[test]
fn lzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 205], OperandSize::Qword)
}

#[test]
fn lzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 643778510, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 148, 219, 206, 71, 95, 38], OperandSize::Qword)
}

#[test]
fn lzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 220], OperandSize::Word)
}

#[test]
fn lzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 6252, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 139, 108, 24], OperandSize::Word)
}

#[test]
fn lzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 226], OperandSize::Dword)
}

#[test]
fn lzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 44, 131], OperandSize::Dword)
}

#[test]
fn lzcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 239], OperandSize::Qword)
}

#[test]
fn lzcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2058942909, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 52, 157, 189, 249, 184, 122], OperandSize::Qword)
}

#[test]
fn lzcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 204], OperandSize::Qword)
}

#[test]
fn lzcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 39], OperandSize::Qword)
}

