use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn tzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 238], OperandSize::Dword)
}

#[test]
fn tzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 635858398, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 140, 199, 222, 109, 230, 37], OperandSize::Dword)
}

#[test]
fn tzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 225], OperandSize::Qword)
}

#[test]
fn tzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2035295753, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 52, 157, 9, 38, 80, 121], OperandSize::Qword)
}

#[test]
fn tzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 218], OperandSize::Dword)
}

#[test]
fn tzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EBX, 1461224024, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 155, 88, 130, 24, 87], OperandSize::Dword)
}

#[test]
fn tzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 221], OperandSize::Qword)
}

#[test]
fn tzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1972612337, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 180, 114, 241, 172, 147, 117], OperandSize::Qword)
}

#[test]
fn tzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 222], OperandSize::Qword)
}

#[test]
fn tzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 36, 120], OperandSize::Qword)
}

