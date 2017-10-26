use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 205], OperandSize::Word)
}

#[test]
fn cmovng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BX, 8901, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 191, 197, 34], OperandSize::Word)
}

#[test]
fn cmovng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 250], OperandSize::Dword)
}

#[test]
fn cmovng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1020808300, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 188, 211, 108, 76, 216, 60], OperandSize::Dword)
}

#[test]
fn cmovng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 219], OperandSize::Qword)
}

#[test]
fn cmovng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 44, 83], OperandSize::Qword)
}

#[test]
fn cmovng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 243], OperandSize::Word)
}

#[test]
fn cmovng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 28624, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 147, 208, 111], OperandSize::Word)
}

#[test]
fn cmovng_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 229], OperandSize::Dword)
}

#[test]
fn cmovng_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 428839423, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 188, 128, 255, 145, 143, 25], OperandSize::Dword)
}

#[test]
fn cmovng_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 217], OperandSize::Qword)
}

#[test]
fn cmovng_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RDI, 407287620, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 175, 68, 183, 70, 24], OperandSize::Qword)
}

#[test]
fn cmovng_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 214], OperandSize::Qword)
}

#[test]
fn cmovng_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 36, 206], OperandSize::Qword)
}

