use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 193], OperandSize::Dword)
}

#[test]
fn orps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 44, 250], OperandSize::Dword)
}

#[test]
fn orps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 206], OperandSize::Qword)
}

#[test]
fn orps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1440630563, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 12, 85, 35, 71, 222, 85], OperandSize::Qword)
}

