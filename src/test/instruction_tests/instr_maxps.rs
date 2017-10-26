use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 217], OperandSize::Dword)
}

#[test]
fn maxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 657276002, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 188, 118, 98, 60, 45, 39], OperandSize::Dword)
}

#[test]
fn maxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 246], OperandSize::Qword)
}

#[test]
fn maxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 30], OperandSize::Qword)
}

