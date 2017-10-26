use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 201], OperandSize::Word)
}

#[test]
fn cmovle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 31081, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 147, 105, 121], OperandSize::Word)
}

#[test]
fn cmovle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 206], OperandSize::Dword)
}

#[test]
fn cmovle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 902662095, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 20, 181, 207, 135, 205, 53], OperandSize::Dword)
}

#[test]
fn cmovle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 249], OperandSize::Qword)
}

#[test]
fn cmovle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 2113107237, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 20, 149, 37, 117, 243, 125], OperandSize::Qword)
}

#[test]
fn cmovle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 231], OperandSize::Word)
}

#[test]
fn cmovle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 28], OperandSize::Word)
}

#[test]
fn cmovle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 236], OperandSize::Dword)
}

#[test]
fn cmovle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 738861181, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 148, 248, 125, 32, 10, 44], OperandSize::Dword)
}

#[test]
fn cmovle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 242], OperandSize::Qword)
}

#[test]
fn cmovle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 958727347, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 36, 93, 179, 4, 37, 57], OperandSize::Qword)
}

#[test]
fn cmovle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 226], OperandSize::Qword)
}

#[test]
fn cmovle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 12, 203], OperandSize::Qword)
}

