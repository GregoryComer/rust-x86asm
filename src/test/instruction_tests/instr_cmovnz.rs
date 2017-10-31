use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnz_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 245], OperandSize::Word)
}

#[test]
fn cmovnz_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 23558, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 144, 6, 92], OperandSize::Word)
}

#[test]
fn cmovnz_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 235], OperandSize::Dword)
}

#[test]
fn cmovnz_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 693432349, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 152, 29, 240, 84, 41], OperandSize::Dword)
}

#[test]
fn cmovnz_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 251], OperandSize::Qword)
}

#[test]
fn cmovnz_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 2139232869, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 156, 129, 101, 26, 130, 127], OperandSize::Qword)
}

#[test]
fn cmovnz_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 226], OperandSize::Word)
}

#[test]
fn cmovnz_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 53, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 99, 53], OperandSize::Word)
}

#[test]
fn cmovnz_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 241], OperandSize::Dword)
}

#[test]
fn cmovnz_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 414957084, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 188, 123, 28, 190, 187, 24], OperandSize::Dword)
}

#[test]
fn cmovnz_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 204], OperandSize::Qword)
}

#[test]
fn cmovnz_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1544725788, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 44, 69, 28, 165, 18, 92], OperandSize::Qword)
}

#[test]
fn cmovnz_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 201], OperandSize::Qword)
}

#[test]
fn cmovnz_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1555431131, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 36, 69, 219, 254, 181, 92], OperandSize::Qword)
}

