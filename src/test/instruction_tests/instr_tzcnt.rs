use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn tzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 217], OperandSize::Dword)
}

#[test]
fn tzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 804200603, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 12, 85, 155, 32, 239, 47], OperandSize::Dword)
}

#[test]
fn tzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 201], OperandSize::Qword)
}

#[test]
fn tzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SI)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 51], OperandSize::Qword)
}

#[test]
fn tzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 249], OperandSize::Dword)
}

#[test]
fn tzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBP)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 41], OperandSize::Dword)
}

#[test]
fn tzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 249], OperandSize::Qword)
}

#[test]
fn tzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 52, 71], OperandSize::Qword)
}

#[test]
fn tzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 249], OperandSize::Qword)
}

#[test]
fn tzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 544738395, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 140, 136, 91, 12, 120, 32], OperandSize::Qword)
}

