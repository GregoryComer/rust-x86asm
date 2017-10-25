use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 228], OperandSize::Word)
}

#[test]
fn cmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 37], OperandSize::Word)
}

#[test]
fn cmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 209], OperandSize::Dword)
}

#[test]
fn cmovbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 599234760, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 36, 157, 200, 152, 183, 35], OperandSize::Dword)
}

#[test]
fn cmovbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 236], OperandSize::Qword)
}

#[test]
fn cmovbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1422182646, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 12, 213, 246, 200, 196, 84], OperandSize::Qword)
}

#[test]
fn cmovbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 241], OperandSize::Word)
}

#[test]
fn cmovbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 61, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 96, 61], OperandSize::Word)
}

#[test]
fn cmovbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 220], OperandSize::Dword)
}

#[test]
fn cmovbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EAX, 364937288, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 176, 72, 128, 192, 21], OperandSize::Dword)
}

#[test]
fn cmovbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 206], OperandSize::Qword)
}

#[test]
fn cmovbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RBX, 1142401691, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 147, 155, 170, 23, 68], OperandSize::Qword)
}

#[test]
fn cmovbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 239], OperandSize::Qword)
}

#[test]
fn cmovbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1892409142, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 36, 149, 54, 223, 203, 112], OperandSize::Qword)
}

