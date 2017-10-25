use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn tzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 235], OperandSize::Dword)
}

#[test]
fn tzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 2104355717, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 172, 139, 133, 235, 109, 125], OperandSize::Dword)
}

#[test]
fn tzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 202], OperandSize::Qword)
}

#[test]
fn tzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SI)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 55], OperandSize::Qword)
}

#[test]
fn tzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 252], OperandSize::Dword)
}

#[test]
fn tzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 402864257, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 44, 69, 129, 56, 3, 24], OperandSize::Dword)
}

#[test]
fn tzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 243], OperandSize::Qword)
}

#[test]
fn tzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RSI, 2079813995, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 166, 107, 113, 247, 123], OperandSize::Qword)
}

#[test]
fn tzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 206], OperandSize::Qword)
}

#[test]
fn tzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1200247671, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 44, 93, 119, 83, 138, 71], OperandSize::Qword)
}

