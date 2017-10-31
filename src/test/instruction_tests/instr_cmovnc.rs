use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 245], OperandSize::Word)
}

#[test]
fn cmovnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DI)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 60], OperandSize::Word)
}

#[test]
fn cmovnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 245], OperandSize::Dword)
}

#[test]
fn cmovnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 36, 139], OperandSize::Dword)
}

#[test]
fn cmovnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 226], OperandSize::Qword)
}

#[test]
fn cmovnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 305021915, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 12, 197, 219, 67, 46, 18], OperandSize::Qword)
}

#[test]
fn cmovnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 226], OperandSize::Word)
}

#[test]
fn cmovnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 32], OperandSize::Word)
}

#[test]
fn cmovnc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 249], OperandSize::Dword)
}

#[test]
fn cmovnc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 2003146166, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 52, 213, 182, 149, 101, 119], OperandSize::Dword)
}

#[test]
fn cmovnc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 249], OperandSize::Qword)
}

#[test]
fn cmovnc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1976667497, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 148, 152, 105, 141, 209, 117], OperandSize::Qword)
}

#[test]
fn cmovnc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 233], OperandSize::Qword)
}

#[test]
fn cmovnc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RCX, 1723054453, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 145, 117, 185, 179, 102], OperandSize::Qword)
}

