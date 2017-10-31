use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovns_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 252], OperandSize::Word)
}

#[test]
fn cmovns_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(DI, 179, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 141, 179, 0], OperandSize::Word)
}

#[test]
fn cmovns_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 226], OperandSize::Dword)
}

#[test]
fn cmovns_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(ECX, 938056779, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 169, 75, 156, 233, 55], OperandSize::Dword)
}

#[test]
fn cmovns_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 243], OperandSize::Qword)
}

#[test]
fn cmovns_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RBX, 244739391, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 179, 63, 109, 150, 14], OperandSize::Qword)
}

#[test]
fn cmovns_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 205], OperandSize::Word)
}

#[test]
fn cmovns_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BP, 12325, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 182, 37, 48], OperandSize::Word)
}

#[test]
fn cmovns_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 227], OperandSize::Dword)
}

#[test]
fn cmovns_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(ECX, 627117704, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 161, 136, 14, 97, 37], OperandSize::Dword)
}

#[test]
fn cmovns_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 242], OperandSize::Qword)
}

#[test]
fn cmovns_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1139354945, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 188, 179, 65, 45, 233, 67], OperandSize::Qword)
}

#[test]
fn cmovns_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 226], OperandSize::Qword)
}

#[test]
fn cmovns_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1727327438, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 188, 95, 206, 236, 244, 102], OperandSize::Qword)
}

