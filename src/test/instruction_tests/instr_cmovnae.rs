use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 201], OperandSize::Word)
}

#[test]
fn cmovnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 2029, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 145, 237, 7], OperandSize::Word)
}

#[test]
fn cmovnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 205], OperandSize::Dword)
}

#[test]
fn cmovnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1525385255, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 28, 181, 39, 136, 235, 90], OperandSize::Dword)
}

#[test]
fn cmovnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 236], OperandSize::Qword)
}

#[test]
fn cmovnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1711495260, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 28, 197, 92, 88, 3, 102], OperandSize::Qword)
}

#[test]
fn cmovnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 209], OperandSize::Word)
}

#[test]
fn cmovnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESI)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 53], OperandSize::Word)
}

#[test]
fn cmovnae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 228], OperandSize::Dword)
}

#[test]
fn cmovnae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 62], OperandSize::Dword)
}

#[test]
fn cmovnae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 228], OperandSize::Qword)
}

#[test]
fn cmovnae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1786246712, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 164, 94, 56, 246, 119, 106], OperandSize::Qword)
}

#[test]
fn cmovnae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 202], OperandSize::Qword)
}

#[test]
fn cmovnae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1202591531, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 148, 87, 43, 23, 174, 71], OperandSize::Qword)
}

