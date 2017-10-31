use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 124, 218], OperandSize::Dword)
}

#[test]
fn vhaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 288076783, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 124, 36, 157, 239, 179, 43, 17], OperandSize::Dword)
}

#[test]
fn vhaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 124, 221], OperandSize::Qword)
}

#[test]
fn vhaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1817631808, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 124, 60, 141, 64, 220, 86, 108], OperandSize::Qword)
}

#[test]
fn vhaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 124, 231], OperandSize::Dword)
}

#[test]
fn vhaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 861288768, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 124, 188, 95, 64, 57, 86, 51], OperandSize::Dword)
}

#[test]
fn vhaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 124, 194], OperandSize::Qword)
}

#[test]
fn vhaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 976968891, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 124, 164, 200, 187, 92, 59, 58], OperandSize::Qword)
}

