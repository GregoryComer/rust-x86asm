use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 228], OperandSize::Word)
}

#[test]
fn cmovge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 17], OperandSize::Word)
}

#[test]
fn cmovge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 203], OperandSize::Dword)
}

#[test]
fn cmovge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EBX, 938033224, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 187, 72, 64, 233, 55], OperandSize::Dword)
}

#[test]
fn cmovge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 215], OperandSize::Qword)
}

#[test]
fn cmovge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(CX)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 15], OperandSize::Qword)
}

#[test]
fn cmovge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 245], OperandSize::Word)
}

#[test]
fn cmovge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BP, 24602, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 182, 26, 96], OperandSize::Word)
}

#[test]
fn cmovge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 235], OperandSize::Dword)
}

#[test]
fn cmovge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 278554715, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 28, 221, 91, 104, 154, 16], OperandSize::Dword)
}

#[test]
fn cmovge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 211], OperandSize::Qword)
}

#[test]
fn cmovge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1011582075, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 28, 93, 123, 132, 75, 60], OperandSize::Qword)
}

#[test]
fn cmovge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 244], OperandSize::Qword)
}

#[test]
fn cmovge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 47], OperandSize::Qword)
}

