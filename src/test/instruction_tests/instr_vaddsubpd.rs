use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 208, 199], OperandSize::Dword)
}

#[test]
fn vaddsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 910695141, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 145, 229, 26, 72, 54], OperandSize::Dword)
}

#[test]
fn vaddsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 208, 239], OperandSize::Qword)
}

#[test]
fn vaddsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 17], OperandSize::Qword)
}

#[test]
fn vaddsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 208, 248], OperandSize::Dword)
}

#[test]
fn vaddsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 208, 58], OperandSize::Dword)
}

#[test]
fn vaddsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 208, 202], OperandSize::Qword)
}

#[test]
fn vaddsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1091677458, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 208, 135, 18, 173, 17, 65], OperandSize::Qword)
}

