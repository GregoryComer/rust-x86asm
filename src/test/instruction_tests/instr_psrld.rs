use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM0)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 208, 19], OperandSize::Dword)
}

#[test]
fn psrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM5)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 213, 96], OperandSize::Qword)
}

#[test]
fn psrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 211, 86], OperandSize::Dword)
}

#[test]
fn psrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 212, 24], OperandSize::Qword)
}

#[test]
fn psrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 195], OperandSize::Dword)
}

#[test]
fn psrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(ESI, 1552851237, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 174, 37, 161, 142, 92], OperandSize::Dword)
}

#[test]
fn psrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 249], OperandSize::Qword)
}

#[test]
fn psrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 28, 176], OperandSize::Qword)
}

#[test]
fn psrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 196], OperandSize::Dword)
}

#[test]
fn psrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 20, 75], OperandSize::Dword)
}

#[test]
fn psrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 211], OperandSize::Qword)
}

#[test]
fn psrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 60, 129], OperandSize::Qword)
}

