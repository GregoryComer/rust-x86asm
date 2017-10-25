use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 206], OperandSize::Dword)
}

#[test]
fn cvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 26], OperandSize::Dword)
}

#[test]
fn cvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 203], OperandSize::Qword)
}

#[test]
fn cvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 20, 121], OperandSize::Qword)
}

