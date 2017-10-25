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
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(BP, 26, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 94, 26], OperandSize::Word)
}

#[test]
fn cmovl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 218], OperandSize::Dword)
}

#[test]
fn cmovl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 95297855, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 172, 114, 63, 33, 174, 5], OperandSize::Dword)
}

#[test]
fn cmovl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 211], OperandSize::Qword)
}

#[test]
fn cmovl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(BX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 25], OperandSize::Qword)
}

#[test]
fn cmovl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 227], OperandSize::Word)
}

#[test]
fn cmovl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 27], OperandSize::Word)
}

#[test]
fn cmovl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 205], OperandSize::Dword)
}

#[test]
fn cmovl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ESI, 938987131, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 174, 123, 206, 247, 55], OperandSize::Dword)
}

#[test]
fn cmovl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 235], OperandSize::Qword)
}

#[test]
fn cmovl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 49], OperandSize::Qword)
}

#[test]
fn cmovl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 243], OperandSize::Qword)
}

#[test]
fn cmovl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 28, 131], OperandSize::Qword)
}

