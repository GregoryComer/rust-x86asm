use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 124, 227], OperandSize::Dword)
}

#[test]
fn vhaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 124, 12, 64], OperandSize::Dword)
}

#[test]
fn vhaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 124, 219], OperandSize::Qword)
}

#[test]
fn vhaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1293856039, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 124, 44, 181, 39, 173, 30, 77], OperandSize::Qword)
}

#[test]
fn vhaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 124, 230], OperandSize::Dword)
}

#[test]
fn vhaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 124, 39], OperandSize::Dword)
}

#[test]
fn vhaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 124, 237], OperandSize::Qword)
}

#[test]
fn vhaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 166899437, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 124, 60, 149, 237, 174, 242, 9], OperandSize::Qword)
}

