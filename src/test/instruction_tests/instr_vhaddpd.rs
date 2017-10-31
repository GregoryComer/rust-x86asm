use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 124, 233], OperandSize::Dword)
}

#[test]
fn vhaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1995621913, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 124, 4, 245, 25, 198, 242, 118], OperandSize::Dword)
}

#[test]
fn vhaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 124, 254], OperandSize::Qword)
}

#[test]
fn vhaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1822666636, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 124, 52, 181, 140, 175, 163, 108], OperandSize::Qword)
}

#[test]
fn vhaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 124, 253], OperandSize::Dword)
}

#[test]
fn vhaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 124, 2], OperandSize::Dword)
}

#[test]
fn vhaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 124, 253], OperandSize::Qword)
}

#[test]
fn vhaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 124, 3], OperandSize::Qword)
}

