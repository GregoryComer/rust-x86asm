use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 227], OperandSize::Word)
}

#[test]
fn cmovae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 94, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 97, 94], OperandSize::Word)
}

#[test]
fn cmovae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 242], OperandSize::Dword)
}

#[test]
fn cmovae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDX, 1126883786, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 146, 202, 225, 42, 67], OperandSize::Dword)
}

#[test]
fn cmovae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 214], OperandSize::Qword)
}

#[test]
fn cmovae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RBX, 1758529053, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 163, 29, 6, 209, 104], OperandSize::Qword)
}

#[test]
fn cmovae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 243], OperandSize::Word)
}

#[test]
fn cmovae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BP, 15767, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 142, 151, 61], OperandSize::Word)
}

#[test]
fn cmovae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 209], OperandSize::Dword)
}

#[test]
fn cmovae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 48], OperandSize::Dword)
}

#[test]
fn cmovae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 231], OperandSize::Qword)
}

#[test]
fn cmovae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1137347214, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 44, 69, 142, 138, 202, 67], OperandSize::Qword)
}

#[test]
fn cmovae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 226], OperandSize::Qword)
}

#[test]
fn cmovae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1824823362, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 156, 89, 66, 152, 196, 108], OperandSize::Qword)
}

