use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 225], OperandSize::Dword)
}

#[test]
fn cvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 515194378, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 12, 85, 10, 62, 181, 30], OperandSize::Dword)
}

#[test]
fn cvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 218], OperandSize::Qword)
}

#[test]
fn cvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 2096986111, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 148, 78, 255, 119, 253, 124], OperandSize::Qword)
}

