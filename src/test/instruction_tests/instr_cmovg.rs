use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 229], OperandSize::Word)
}

#[test]
fn cmovg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 26], OperandSize::Word)
}

#[test]
fn cmovg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 255], OperandSize::Dword)
}

#[test]
fn cmovg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 874337309, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 52, 117, 29, 84, 29, 52], OperandSize::Dword)
}

#[test]
fn cmovg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 227], OperandSize::Qword)
}

#[test]
fn cmovg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1767627306, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 140, 70, 42, 218, 91, 105], OperandSize::Qword)
}

#[test]
fn cmovg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 206], OperandSize::Word)
}

#[test]
fn cmovg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 16], OperandSize::Word)
}

#[test]
fn cmovg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 213], OperandSize::Dword)
}

#[test]
fn cmovg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(ECX, 517979577, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 177, 185, 189, 223, 30], OperandSize::Dword)
}

#[test]
fn cmovg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 247], OperandSize::Qword)
}

#[test]
fn cmovg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 560007121, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 172, 113, 209, 7, 97, 33], OperandSize::Qword)
}

#[test]
fn cmovg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 218], OperandSize::Qword)
}

#[test]
fn cmovg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1191773575, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 180, 123, 135, 5, 9, 71], OperandSize::Qword)
}

