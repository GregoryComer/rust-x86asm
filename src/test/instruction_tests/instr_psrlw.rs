use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 211, 119], OperandSize::Dword)
}

#[test]
fn psrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 209, 117], OperandSize::Qword)
}

#[test]
fn psrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 213, 90], OperandSize::Dword)
}

#[test]
fn psrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 212, 60], OperandSize::Qword)
}

#[test]
fn psrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 213], OperandSize::Dword)
}

#[test]
fn psrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 147693067, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 4, 117, 11, 158, 205, 8], OperandSize::Dword)
}

#[test]
fn psrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 214], OperandSize::Qword)
}

#[test]
fn psrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1435711350, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 4, 245, 118, 55, 147, 85], OperandSize::Qword)
}

#[test]
fn psrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 251], OperandSize::Dword)
}

#[test]
fn psrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1478017859, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 145, 67, 195, 24, 88], OperandSize::Dword)
}

#[test]
fn psrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 246], OperandSize::Qword)
}

#[test]
fn psrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1384705984, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 140, 255, 192, 239, 136, 82], OperandSize::Qword)
}

