use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 202], OperandSize::Word)
}

#[test]
fn cmovl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(CX)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 15], OperandSize::Word)
}

#[test]
fn cmovl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 225], OperandSize::Dword)
}

#[test]
fn cmovl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 1649426760, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 152, 72, 65, 80, 98], OperandSize::Dword)
}

#[test]
fn cmovl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 202], OperandSize::Qword)
}

#[test]
fn cmovl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1413979039, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 148, 194, 159, 155, 71, 84], OperandSize::Qword)
}

#[test]
fn cmovl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 254], OperandSize::Word)
}

#[test]
fn cmovl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 8], OperandSize::Word)
}

#[test]
fn cmovl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 253], OperandSize::Dword)
}

#[test]
fn cmovl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 44, 123], OperandSize::Dword)
}

#[test]
fn cmovl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 203], OperandSize::Qword)
}

#[test]
fn cmovl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 46], OperandSize::Qword)
}

#[test]
fn cmovl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 202], OperandSize::Qword)
}

#[test]
fn cmovl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RCX, 1637010882, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 145, 194, 205, 146, 97], OperandSize::Qword)
}

