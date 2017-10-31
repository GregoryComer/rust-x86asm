use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 218], OperandSize::Word)
}

#[test]
fn cmovnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 47], OperandSize::Word)
}

#[test]
fn cmovnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 250], OperandSize::Dword)
}

#[test]
fn cmovnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 60, 179], OperandSize::Dword)
}

#[test]
fn cmovnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 213], OperandSize::Qword)
}

#[test]
fn cmovnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 44, 81], OperandSize::Qword)
}

#[test]
fn cmovnle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 223], OperandSize::Word)
}

#[test]
fn cmovnle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(BP, 28005, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 174, 101, 109], OperandSize::Word)
}

#[test]
fn cmovnle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 214], OperandSize::Dword)
}

#[test]
fn cmovnle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(ESI, 681969045, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 166, 149, 5, 166, 40], OperandSize::Dword)
}

#[test]
fn cmovnle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 249], OperandSize::Qword)
}

#[test]
fn cmovnle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 742226272, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 156, 128, 96, 121, 61, 44], OperandSize::Qword)
}

#[test]
fn cmovnle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 218], OperandSize::Qword)
}

#[test]
fn cmovnle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1458598602, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 156, 73, 202, 114, 240, 86], OperandSize::Qword)
}

