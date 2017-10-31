use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 230, 7], OperandSize::Dword)
}

#[test]
fn roundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1516934844, Some(OperandSize::Qword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 180, 158, 188, 150, 106, 90, 66], OperandSize::Dword)
}

#[test]
fn roundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 220, 56], OperandSize::Qword)
}

#[test]
fn roundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 140869313, Some(OperandSize::Qword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 52, 117, 193, 126, 101, 8, 116], OperandSize::Qword)
}

