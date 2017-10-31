use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 219, 29], OperandSize::Dword)
}

#[test]
fn roundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1455967715, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 140, 182, 227, 77, 200, 86, 64], OperandSize::Dword)
}

#[test]
fn roundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 208, 117], OperandSize::Qword)
}

#[test]
fn roundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 2092440980, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 176, 148, 29, 184, 124, 6], OperandSize::Qword)
}

