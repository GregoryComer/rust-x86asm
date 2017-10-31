use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 235], OperandSize::Word)
}

#[test]
fn cmovge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 21], OperandSize::Word)
}

#[test]
fn cmovge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 233], OperandSize::Dword)
}

#[test]
fn cmovge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(EAX, 9819048, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 160, 168, 211, 149, 0], OperandSize::Dword)
}

#[test]
fn cmovge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 210], OperandSize::Qword)
}

#[test]
fn cmovge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 2088270223, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 28, 213, 143, 121, 120, 124], OperandSize::Qword)
}

#[test]
fn cmovge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 231], OperandSize::Word)
}

#[test]
fn cmovge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BP, 292, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 158, 36, 1], OperandSize::Word)
}

#[test]
fn cmovge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 204], OperandSize::Dword)
}

#[test]
fn cmovge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EDX, 1994242943, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 178, 127, 187, 221, 118], OperandSize::Dword)
}

#[test]
fn cmovge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 247], OperandSize::Qword)
}

#[test]
fn cmovge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RCX, 33295028, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 153, 180, 10, 252, 1], OperandSize::Qword)
}

#[test]
fn cmovge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 225], OperandSize::Qword)
}

#[test]
fn cmovge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 23], OperandSize::Qword)
}

