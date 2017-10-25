use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 219], OperandSize::Word)
}

#[test]
fn cmovle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 29], OperandSize::Word)
}

#[test]
fn cmovle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 254], OperandSize::Dword)
}

#[test]
fn cmovle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(ECX, 546798339, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 145, 3, 123, 151, 32], OperandSize::Dword)
}

#[test]
fn cmovle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 223], OperandSize::Qword)
}

#[test]
fn cmovle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 16], OperandSize::Qword)
}

#[test]
fn cmovle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 241], OperandSize::Word)
}

#[test]
fn cmovle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 50], OperandSize::Word)
}

#[test]
fn cmovle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 207], OperandSize::Dword)
}

#[test]
fn cmovle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EBX, 1268819145, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 171, 201, 164, 160, 75], OperandSize::Dword)
}

#[test]
fn cmovle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 217], OperandSize::Qword)
}

#[test]
fn cmovle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 33], OperandSize::Qword)
}

#[test]
fn cmovle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 207], OperandSize::Qword)
}

#[test]
fn cmovle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1875953656, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 60, 205, 248, 199, 208, 111], OperandSize::Qword)
}

