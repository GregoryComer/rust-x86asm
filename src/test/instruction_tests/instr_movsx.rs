use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 226], OperandSize::Word)
}

#[test]
fn movsx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20255, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 138, 31, 79], OperandSize::Word)
}

#[test]
fn movsx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 210], OperandSize::Dword)
}

#[test]
fn movsx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 52, 242], OperandSize::Dword)
}

#[test]
fn movsx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 201], OperandSize::Qword)
}

#[test]
fn movsx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(BX)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 24], OperandSize::Qword)
}

#[test]
fn movsx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 227], OperandSize::Word)
}

#[test]
fn movsx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 47], OperandSize::Word)
}

#[test]
fn movsx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 242], OperandSize::Dword)
}

#[test]
fn movsx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 36, 91], OperandSize::Dword)
}

#[test]
fn movsx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 226], OperandSize::Qword)
}

#[test]
fn movsx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1589290534, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 188, 114, 38, 166, 186, 94], OperandSize::Qword)
}

#[test]
fn movsx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 227], OperandSize::Qword)
}

#[test]
fn movsx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 41341863, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 148, 159, 167, 211, 118, 2], OperandSize::Qword)
}

#[test]
fn movsx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 239], OperandSize::Word)
}

#[test]
fn movsx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 62, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 106, 62], OperandSize::Word)
}

#[test]
fn movsx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 217], OperandSize::Dword)
}

#[test]
fn movsx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1875211593, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 20, 69, 73, 117, 197, 111], OperandSize::Dword)
}

#[test]
fn movsx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 202], OperandSize::Qword)
}

#[test]
fn movsx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RCX, 521451338, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 145, 74, 183, 20, 31], OperandSize::Qword)
}

#[test]
fn movsx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 245], OperandSize::Qword)
}

#[test]
fn movsx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDI, 234723861, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 183, 21, 154, 253, 13], OperandSize::Qword)
}

