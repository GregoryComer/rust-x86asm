use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 206], OperandSize::Word)
}

#[test]
fn cmovge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(SI, 251, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 156, 251, 0], OperandSize::Word)
}

#[test]
fn cmovge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 253], OperandSize::Dword)
}

#[test]
fn cmovge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BP)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 47], OperandSize::Dword)
}

#[test]
fn cmovge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 245], OperandSize::Qword)
}

#[test]
fn cmovge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(RBX, 1564921342, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 155, 254, 205, 70, 93], OperandSize::Qword)
}

#[test]
fn cmovge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 254], OperandSize::Word)
}

#[test]
fn cmovge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 33], OperandSize::Word)
}

#[test]
fn cmovge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 247], OperandSize::Dword)
}

#[test]
fn cmovge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 60, 251], OperandSize::Dword)
}

#[test]
fn cmovge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 233], OperandSize::Qword)
}

#[test]
fn cmovge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RCX, 2056796769, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 177, 97, 58, 152, 122], OperandSize::Qword)
}

#[test]
fn cmovge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 220], OperandSize::Qword)
}

#[test]
fn cmovge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1859845074, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 156, 75, 210, 251, 218, 110], OperandSize::Qword)
}

