use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn tzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 210], OperandSize::Dword)
}

#[test]
fn tzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 66832509, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 140, 119, 125, 200, 251, 3], OperandSize::Dword)
}

#[test]
fn tzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 246], OperandSize::Qword)
}

#[test]
fn tzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 8], OperandSize::Qword)
}

#[test]
fn tzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 243], OperandSize::Dword)
}

#[test]
fn tzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 60, 126], OperandSize::Dword)
}

#[test]
fn tzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 226], OperandSize::Qword)
}

#[test]
fn tzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 368000026, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 20, 77, 26, 60, 239, 21], OperandSize::Qword)
}

#[test]
fn tzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 235], OperandSize::Qword)
}

#[test]
fn tzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 42], OperandSize::Qword)
}

