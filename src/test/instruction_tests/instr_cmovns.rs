use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovns_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 244], OperandSize::Word)
}

#[test]
fn cmovns_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BX, 182, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 183, 182, 0], OperandSize::Word)
}

#[test]
fn cmovns_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 253], OperandSize::Dword)
}

#[test]
fn cmovns_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(ESI, 981892740, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 158, 132, 126, 134, 58], OperandSize::Dword)
}

#[test]
fn cmovns_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 238], OperandSize::Qword)
}

#[test]
fn cmovns_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1751861828, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 172, 187, 68, 74, 107, 104], OperandSize::Qword)
}

#[test]
fn cmovns_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 249], OperandSize::Word)
}

#[test]
fn cmovns_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 57], OperandSize::Word)
}

#[test]
fn cmovns_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 218], OperandSize::Dword)
}

#[test]
fn cmovns_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 23], OperandSize::Dword)
}

#[test]
fn cmovns_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 226], OperandSize::Qword)
}

#[test]
fn cmovns_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 2111408141, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 172, 144, 13, 136, 217, 125], OperandSize::Qword)
}

#[test]
fn cmovns_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 222], OperandSize::Qword)
}

#[test]
fn cmovns_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1151160959, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 12, 93, 127, 82, 157, 68], OperandSize::Qword)
}

