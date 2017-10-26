use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 222, 92], OperandSize::Dword)
}

#[test]
fn cmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 274189780, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 12, 205, 212, 205, 87, 16, 16], OperandSize::Dword)
}

#[test]
fn cmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 255, 56], OperandSize::Qword)
}

#[test]
fn cmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 12, 176, 5], OperandSize::Qword)
}

