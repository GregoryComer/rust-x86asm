use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 124, 242], OperandSize::Dword)
}

#[test]
fn vhaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 124, 58], OperandSize::Dword)
}

#[test]
fn vhaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 124, 255], OperandSize::Qword)
}

#[test]
fn vhaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 981914958, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 124, 134, 78, 213, 134, 58], OperandSize::Qword)
}

#[test]
fn vhaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 124, 255], OperandSize::Dword)
}

#[test]
fn vhaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 10253014, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 124, 153, 214, 114, 156, 0], OperandSize::Dword)
}

#[test]
fn vhaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 124, 206], OperandSize::Qword)
}

#[test]
fn vhaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 110549720, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 124, 137, 216, 218, 150, 6], OperandSize::Qword)
}

