use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 200], OperandSize::Dword)
}

#[test]
fn haddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 639076830, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 132, 119, 222, 137, 23, 38], OperandSize::Dword)
}

#[test]
fn haddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 234], OperandSize::Qword)
}

#[test]
fn haddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 932093432, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 28, 181, 248, 157, 142, 55], OperandSize::Qword)
}

