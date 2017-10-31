use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 209], OperandSize::Word)
}

#[test]
fn cmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 45, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 113, 45], OperandSize::Word)
}

#[test]
fn cmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 254], OperandSize::Dword)
}

#[test]
fn cmovnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(EBX, 1158337246, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 163, 222, 210, 10, 69], OperandSize::Dword)
}

#[test]
fn cmovnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 210], OperandSize::Qword)
}

#[test]
fn cmovnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(CX)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 15], OperandSize::Qword)
}

#[test]
fn cmovnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 235], OperandSize::Word)
}

#[test]
fn cmovnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(DI, 130, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 173, 130, 0], OperandSize::Word)
}

#[test]
fn cmovnbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 241], OperandSize::Dword)
}

#[test]
fn cmovnbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 44, 211], OperandSize::Dword)
}

#[test]
fn cmovnbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 213], OperandSize::Qword)
}

#[test]
fn cmovnbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 36, 135], OperandSize::Qword)
}

#[test]
fn cmovnbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 230], OperandSize::Qword)
}

#[test]
fn cmovnbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RBX, 308814488, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 155, 152, 34, 104, 18], OperandSize::Qword)
}

