use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 220], OperandSize::Word)
}

#[test]
fn cmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17739, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 145, 75, 69], OperandSize::Word)
}

#[test]
fn cmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 221], OperandSize::Dword)
}

#[test]
fn cmovnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 467927116, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 60, 205, 76, 0, 228, 27], OperandSize::Dword)
}

#[test]
fn cmovnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 222], OperandSize::Qword)
}

#[test]
fn cmovnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 2013975579, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 12, 77, 27, 212, 10, 120], OperandSize::Qword)
}

#[test]
fn cmovnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 223], OperandSize::Word)
}

#[test]
fn cmovnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 99, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 80, 99], OperandSize::Word)
}

#[test]
fn cmovnbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 236], OperandSize::Dword)
}

#[test]
fn cmovnbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 28, 88], OperandSize::Dword)
}

#[test]
fn cmovnbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 206], OperandSize::Qword)
}

#[test]
fn cmovnbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1800471230, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 164, 79, 190, 2, 81, 107], OperandSize::Qword)
}

#[test]
fn cmovnbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 243], OperandSize::Qword)
}

#[test]
fn cmovnbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 129787156, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 12, 117, 20, 101, 188, 7], OperandSize::Qword)
}

