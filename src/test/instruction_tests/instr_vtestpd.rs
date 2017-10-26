use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 243], OperandSize::Dword)
}

#[test]
fn vtestpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 30], OperandSize::Dword)
}

#[test]
fn vtestpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 211], OperandSize::Qword)
}

#[test]
fn vtestpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 40], OperandSize::Qword)
}

#[test]
fn vtestpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 236], OperandSize::Dword)
}

#[test]
fn vtestpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 35], OperandSize::Dword)
}

#[test]
fn vtestpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 219], OperandSize::Qword)
}

#[test]
fn vtestpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1611805848, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 4, 213, 152, 52, 18, 96], OperandSize::Qword)
}

