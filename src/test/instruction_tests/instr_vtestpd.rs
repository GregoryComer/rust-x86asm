use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 240], OperandSize::Dword)
}

#[test]
fn vtestpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 681730245, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 60, 141, 197, 96, 162, 40], OperandSize::Dword)
}

#[test]
fn vtestpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 233], OperandSize::Qword)
}

#[test]
fn vtestpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 57], OperandSize::Qword)
}

#[test]
fn vtestpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 255], OperandSize::Dword)
}

#[test]
fn vtestpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 44, 176], OperandSize::Dword)
}

#[test]
fn vtestpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 226], OperandSize::Qword)
}

#[test]
fn vtestpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 42], OperandSize::Qword)
}

