use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 251], OperandSize::Word)
}

#[test]
fn cmovnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 51], OperandSize::Word)
}

#[test]
fn cmovnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 228], OperandSize::Dword)
}

#[test]
fn cmovnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1054846170, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 180, 241, 218, 172, 223, 62], OperandSize::Dword)
}

#[test]
fn cmovnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 201], OperandSize::Qword)
}

#[test]
fn cmovnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(RDI, 700959500, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 159, 12, 203, 199, 41], OperandSize::Qword)
}

#[test]
fn cmovnle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 217], OperandSize::Word)
}

#[test]
fn cmovnle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 14111, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 185, 31, 55], OperandSize::Word)
}

#[test]
fn cmovnle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 235], OperandSize::Dword)
}

#[test]
fn cmovnle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ECX, 481144180, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 169, 116, 173, 173, 28], OperandSize::Dword)
}

#[test]
fn cmovnle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 222], OperandSize::Qword)
}

#[test]
fn cmovnle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 48], OperandSize::Qword)
}

#[test]
fn cmovnle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 222], OperandSize::Qword)
}

#[test]
fn cmovnle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RDI, 1619889309, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 159, 157, 140, 141, 96], OperandSize::Qword)
}

