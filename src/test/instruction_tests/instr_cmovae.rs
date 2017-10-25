use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 253], OperandSize::Word)
}

#[test]
fn cmovae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 86, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 126, 86], OperandSize::Word)
}

#[test]
fn cmovae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 207], OperandSize::Dword)
}

#[test]
fn cmovae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(EBX, 34959983, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 179, 111, 114, 21, 2], OperandSize::Dword)
}

#[test]
fn cmovae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 237], OperandSize::Qword)
}

#[test]
fn cmovae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1633844018, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 60, 69, 50, 123, 98, 97], OperandSize::Qword)
}

#[test]
fn cmovae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 227], OperandSize::Word)
}

#[test]
fn cmovae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 137, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 154, 137, 0], OperandSize::Word)
}

#[test]
fn cmovae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 223], OperandSize::Dword)
}

#[test]
fn cmovae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 28, 177], OperandSize::Dword)
}

#[test]
fn cmovae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 246], OperandSize::Qword)
}

#[test]
fn cmovae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 60, 194], OperandSize::Qword)
}

#[test]
fn cmovae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 225], OperandSize::Qword)
}

#[test]
fn cmovae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1005022304, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 36, 93, 96, 108, 231, 59], OperandSize::Qword)
}

