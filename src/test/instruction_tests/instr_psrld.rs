use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM4)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 212, 63], OperandSize::Dword)
}

#[test]
fn psrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 210, 15], OperandSize::Qword)
}

#[test]
fn psrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 211, 21], OperandSize::Dword)
}

#[test]
fn psrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 210, 1], OperandSize::Qword)
}

#[test]
fn psrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 230], OperandSize::Dword)
}

#[test]
fn psrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 15], OperandSize::Dword)
}

#[test]
fn psrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 232], OperandSize::Qword)
}

#[test]
fn psrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 60, 202], OperandSize::Qword)
}

#[test]
fn psrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 240], OperandSize::Dword)
}

#[test]
fn psrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1842527233, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 167, 1, 188, 210, 109], OperandSize::Dword)
}

#[test]
fn psrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 209], OperandSize::Qword)
}

#[test]
fn psrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 433609847, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 140, 72, 119, 92, 216, 25], OperandSize::Qword)
}

