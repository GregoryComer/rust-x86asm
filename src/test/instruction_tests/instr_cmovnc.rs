use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 219], OperandSize::Word)
}

#[test]
fn cmovnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 148, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 147, 148, 0], OperandSize::Word)
}

#[test]
fn cmovnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 209], OperandSize::Dword)
}

#[test]
fn cmovnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BP)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 40], OperandSize::Dword)
}

#[test]
fn cmovnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 205], OperandSize::Qword)
}

#[test]
fn cmovnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 552179716, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 164, 242, 4, 152, 233, 32], OperandSize::Qword)
}

#[test]
fn cmovnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 212], OperandSize::Word)
}

#[test]
fn cmovnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 5136, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 145, 16, 20], OperandSize::Word)
}

#[test]
fn cmovnc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 214], OperandSize::Dword)
}

#[test]
fn cmovnc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 44, 130], OperandSize::Dword)
}

#[test]
fn cmovnc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 218], OperandSize::Qword)
}

#[test]
fn cmovnc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1436851456, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 36, 141, 0, 157, 164, 85], OperandSize::Qword)
}

#[test]
fn cmovnc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 204], OperandSize::Qword)
}

#[test]
fn cmovnc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1933052244, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 172, 222, 84, 9, 56, 115], OperandSize::Qword)
}

