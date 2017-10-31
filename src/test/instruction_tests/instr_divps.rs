use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 196], OperandSize::Dword)
}

#[test]
fn divps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 59], OperandSize::Dword)
}

#[test]
fn divps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 246], OperandSize::Qword)
}

#[test]
fn divps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 38], OperandSize::Qword)
}

