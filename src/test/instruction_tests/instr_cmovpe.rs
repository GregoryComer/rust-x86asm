use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovpe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 211], OperandSize::Word)
}

#[test]
fn cmovpe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 193, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 169, 193, 0], OperandSize::Word)
}

#[test]
fn cmovpe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 211], OperandSize::Dword)
}

#[test]
fn cmovpe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1338224945, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 36, 197, 49, 177, 195, 79], OperandSize::Dword)
}

#[test]
fn cmovpe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 229], OperandSize::Qword)
}

#[test]
fn cmovpe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RCX, 1779183959, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 169, 87, 49, 12, 106], OperandSize::Qword)
}

#[test]
fn cmovpe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 222], OperandSize::Word)
}

#[test]
fn cmovpe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 4367, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 162, 15, 17], OperandSize::Word)
}

#[test]
fn cmovpe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 241], OperandSize::Dword)
}

#[test]
fn cmovpe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EDX, 880521988, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 154, 4, 179, 123, 52], OperandSize::Dword)
}

#[test]
fn cmovpe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 245], OperandSize::Qword)
}

#[test]
fn cmovpe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1492236704, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 12, 221, 160, 185, 241, 88], OperandSize::Qword)
}

#[test]
fn cmovpe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 207], OperandSize::Qword)
}

#[test]
fn cmovpe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 25], OperandSize::Qword)
}

