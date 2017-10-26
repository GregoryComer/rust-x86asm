use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 245], OperandSize::Word)
}

#[test]
fn cmovnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 40], OperandSize::Word)
}

#[test]
fn cmovnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 251], OperandSize::Dword)
}

#[test]
fn cmovnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(ESI, 1747344740, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 190, 100, 93, 38, 104], OperandSize::Dword)
}

#[test]
fn cmovnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 222], OperandSize::Qword)
}

#[test]
fn cmovnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RDI, 886487405, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 143, 109, 185, 214, 52], OperandSize::Qword)
}

#[test]
fn cmovnle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 236], OperandSize::Word)
}

#[test]
fn cmovnle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4327, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 139, 231, 16], OperandSize::Word)
}

#[test]
fn cmovnle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 247], OperandSize::Dword)
}

#[test]
fn cmovnle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1516577135, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 148, 215, 111, 33, 101, 90], OperandSize::Dword)
}

#[test]
fn cmovnle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 227], OperandSize::Qword)
}

#[test]
fn cmovnle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 40], OperandSize::Qword)
}

#[test]
fn cmovnle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 204], OperandSize::Qword)
}

#[test]
fn cmovnle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 431073690, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 164, 187, 154, 169, 177, 25], OperandSize::Qword)
}

