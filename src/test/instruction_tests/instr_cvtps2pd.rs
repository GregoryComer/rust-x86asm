use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 246], OperandSize::Dword)
}

#[test]
fn cvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1714375238, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 172, 135, 70, 74, 47, 102], OperandSize::Dword)
}

#[test]
fn cvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 205], OperandSize::Qword)
}

#[test]
fn cvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 381580003, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 4, 85, 227, 114, 190, 22], OperandSize::Qword)
}

