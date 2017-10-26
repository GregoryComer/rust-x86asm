use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM6)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 214, 95], OperandSize::Dword)
}

#[test]
fn psrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM5)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 213, 58], OperandSize::Qword)
}

#[test]
fn psrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 208, 36], OperandSize::Dword)
}

#[test]
fn psrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 212, 102], OperandSize::Qword)
}

#[test]
fn psrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 216], OperandSize::Dword)
}

#[test]
fn psrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1866821587, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 44, 253, 211, 111, 69, 111], OperandSize::Dword)
}

#[test]
fn psrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 213], OperandSize::Qword)
}

#[test]
fn psrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1700014064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 52, 253, 240, 39, 84, 101], OperandSize::Qword)
}

#[test]
fn psrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 253], OperandSize::Dword)
}

#[test]
fn psrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 60, 187], OperandSize::Dword)
}

#[test]
fn psrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 226], OperandSize::Qword)
}

#[test]
fn psrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 731573524, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 143, 20, 237, 154, 43], OperandSize::Qword)
}

