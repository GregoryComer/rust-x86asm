use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 243], OperandSize::Word)
}

#[test]
fn movsx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(BP, 61, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 78, 61], OperandSize::Word)
}

#[test]
fn movsx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 242], OperandSize::Dword)
}

#[test]
fn movsx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 60, 91], OperandSize::Dword)
}

#[test]
fn movsx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(CX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 203], OperandSize::Qword)
}

#[test]
fn movsx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 50], OperandSize::Qword)
}

#[test]
fn movsx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 249], OperandSize::Word)
}

#[test]
fn movsx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 195, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 177, 195, 0], OperandSize::Word)
}

#[test]
fn movsx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 218], OperandSize::Dword)
}

#[test]
fn movsx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 724106941, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 156, 246, 189, 254, 40, 43], OperandSize::Dword)
}

#[test]
fn movsx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 225], OperandSize::Qword)
}

#[test]
fn movsx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RSI, 1863279886, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 150, 14, 101, 15, 111], OperandSize::Qword)
}

#[test]
fn movsx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 226], OperandSize::Qword)
}

#[test]
fn movsx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 57], OperandSize::Qword)
}

#[test]
fn movsx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 218], OperandSize::Word)
}

#[test]
fn movsx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 47], OperandSize::Word)
}

#[test]
fn movsx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 244], OperandSize::Dword)
}

#[test]
fn movsx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 12, 139], OperandSize::Dword)
}

#[test]
fn movsx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 241], OperandSize::Qword)
}

#[test]
fn movsx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 24], OperandSize::Qword)
}

#[test]
fn movsx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 212], OperandSize::Qword)
}

#[test]
fn movsx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1042481461, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 188, 254, 53, 1, 35, 62], OperandSize::Qword)
}

