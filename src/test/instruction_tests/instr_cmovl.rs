use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 241], OperandSize::Word)
}

#[test]
fn cmovl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 11679, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 154, 159, 45], OperandSize::Word)
}

#[test]
fn cmovl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 235], OperandSize::Dword)
}

#[test]
fn cmovl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(ESI, 495361646, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 150, 110, 158, 134, 29], OperandSize::Dword)
}

#[test]
fn cmovl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 236], OperandSize::Qword)
}

#[test]
fn cmovl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 489008472, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 180, 137, 88, 173, 37, 29], OperandSize::Qword)
}

#[test]
fn cmovl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 222], OperandSize::Word)
}

#[test]
fn cmovl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 86, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 83, 86], OperandSize::Word)
}

#[test]
fn cmovl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 214], OperandSize::Dword)
}

#[test]
fn cmovl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EBX, 2023979562, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 179, 42, 122, 163, 120], OperandSize::Dword)
}

#[test]
fn cmovl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 233], OperandSize::Qword)
}

#[test]
fn cmovl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1490841659, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 148, 248, 59, 112, 220, 88], OperandSize::Qword)
}

#[test]
fn cmovl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 252], OperandSize::Qword)
}

#[test]
fn cmovl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 2066206367, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 148, 94, 159, 206, 39, 123], OperandSize::Qword)
}

