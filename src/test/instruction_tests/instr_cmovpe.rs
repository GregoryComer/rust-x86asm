use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovpe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 223], OperandSize::Word)
}

#[test]
fn cmovpe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 48], OperandSize::Word)
}

#[test]
fn cmovpe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 253], OperandSize::Dword)
}

#[test]
fn cmovpe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 24], OperandSize::Dword)
}

#[test]
fn cmovpe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 219], OperandSize::Qword)
}

#[test]
fn cmovpe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1349165219, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 140, 74, 163, 160, 106, 80], OperandSize::Qword)
}

#[test]
fn cmovpe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 250], OperandSize::Word)
}

#[test]
fn cmovpe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 86, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 91, 86], OperandSize::Word)
}

#[test]
fn cmovpe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 253], OperandSize::Dword)
}

#[test]
fn cmovpe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 60, 118], OperandSize::Dword)
}

#[test]
fn cmovpe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 218], OperandSize::Qword)
}

#[test]
fn cmovpe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 102754086, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 44, 125, 38, 231, 31, 6], OperandSize::Qword)
}

#[test]
fn cmovpe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 255], OperandSize::Qword)
}

#[test]
fn cmovpe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 44, 113], OperandSize::Qword)
}

