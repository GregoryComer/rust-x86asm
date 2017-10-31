use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn popcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 211], OperandSize::Word)
}

#[test]
fn popcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 6146, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 168, 2, 24], OperandSize::Word)
}

#[test]
fn popcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 250], OperandSize::Dword)
}

#[test]
fn popcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EBX, 446123909, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 187, 133, 79, 151, 26], OperandSize::Dword)
}

#[test]
fn popcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 210], OperandSize::Qword)
}

#[test]
fn popcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1633915245, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 140, 202, 109, 145, 99, 97], OperandSize::Qword)
}

#[test]
fn popcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 238], OperandSize::Word)
}

#[test]
fn popcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 232, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 146, 232, 0], OperandSize::Word)
}

#[test]
fn popcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 242], OperandSize::Dword)
}

#[test]
fn popcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EDI, 2120427418, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 175, 154, 39, 99, 126], OperandSize::Dword)
}

#[test]
fn popcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 249], OperandSize::Qword)
}

#[test]
fn popcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 20, 199], OperandSize::Qword)
}

#[test]
fn popcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 236], OperandSize::Qword)
}

#[test]
fn popcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 60, 240], OperandSize::Qword)
}

