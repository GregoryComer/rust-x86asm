use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 237], OperandSize::Word)
}

#[test]
fn cmovl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(Memory(27711, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 38, 63, 108], OperandSize::Word)
}

#[test]
fn cmovl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 231], OperandSize::Dword)
}

#[test]
fn cmovl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1303374715, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 164, 177, 123, 235, 175, 77], OperandSize::Dword)
}

#[test]
fn cmovl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 202], OperandSize::Qword)
}

#[test]
fn cmovl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RDI, 444420833, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 167, 225, 82, 125, 26], OperandSize::Qword)
}

#[test]
fn cmovl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 215], OperandSize::Word)
}

#[test]
fn cmovl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 49], OperandSize::Word)
}

#[test]
fn cmovl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 218], OperandSize::Dword)
}

#[test]
fn cmovl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 12, 94], OperandSize::Dword)
}

#[test]
fn cmovl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 252], OperandSize::Qword)
}

#[test]
fn cmovl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 989574600, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 20, 149, 200, 181, 251, 58], OperandSize::Qword)
}

#[test]
fn cmovl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 222], OperandSize::Qword)
}

#[test]
fn cmovl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 495862169, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 156, 177, 153, 65, 142, 29], OperandSize::Qword)
}

