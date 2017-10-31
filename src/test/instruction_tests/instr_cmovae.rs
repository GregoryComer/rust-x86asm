use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 207], OperandSize::Word)
}

#[test]
fn cmovae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BX, 27976, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 183, 72, 109], OperandSize::Word)
}

#[test]
fn cmovae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 228], OperandSize::Dword)
}

#[test]
fn cmovae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 52, 75], OperandSize::Dword)
}

#[test]
fn cmovae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 254], OperandSize::Qword)
}

#[test]
fn cmovae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1304709571, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 148, 146, 195, 73, 196, 77], OperandSize::Qword)
}

#[test]
fn cmovae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 209], OperandSize::Word)
}

#[test]
fn cmovae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13221, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 139, 165, 51], OperandSize::Word)
}

#[test]
fn cmovae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 226], OperandSize::Dword)
}

#[test]
fn cmovae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 411681819, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 140, 202, 27, 196, 137, 24], OperandSize::Dword)
}

#[test]
fn cmovae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 220], OperandSize::Qword)
}

#[test]
fn cmovae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 18], OperandSize::Qword)
}

#[test]
fn cmovae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 228], OperandSize::Qword)
}

#[test]
fn cmovae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 12, 87], OperandSize::Qword)
}

