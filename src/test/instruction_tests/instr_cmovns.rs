use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovns_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 225], OperandSize::Word)
}

#[test]
fn cmovns_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 47], OperandSize::Word)
}

#[test]
fn cmovns_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 217], OperandSize::Dword)
}

#[test]
fn cmovns_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SP)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 38], OperandSize::Dword)
}

#[test]
fn cmovns_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 203], OperandSize::Qword)
}

#[test]
fn cmovns_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RSI, 189726465, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 190, 1, 255, 78, 11], OperandSize::Qword)
}

#[test]
fn cmovns_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 237], OperandSize::Word)
}

#[test]
fn cmovns_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BX, 24694, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 143, 118, 96], OperandSize::Word)
}

#[test]
fn cmovns_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 252], OperandSize::Dword)
}

#[test]
fn cmovns_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 60, 144], OperandSize::Dword)
}

#[test]
fn cmovns_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 225], OperandSize::Qword)
}

#[test]
fn cmovns_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RDX, 498428952, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 186, 24, 108, 181, 29], OperandSize::Qword)
}

#[test]
fn cmovns_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 250], OperandSize::Qword)
}

#[test]
fn cmovns_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1596582835, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 172, 83, 179, 235, 41, 95], OperandSize::Qword)
}

