use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 219], OperandSize::Dword)
}

#[test]
fn vtestpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1944983365, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 140, 254, 69, 23, 238, 115], OperandSize::Dword)
}

#[test]
fn vtestpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 247], OperandSize::Qword)
}

#[test]
fn vtestpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 20, 187], OperandSize::Qword)
}

#[test]
fn vtestpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 207], OperandSize::Dword)
}

#[test]
fn vtestpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 427623599, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 20, 69, 175, 4, 125, 25], OperandSize::Dword)
}

#[test]
fn vtestpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 248], OperandSize::Qword)
}

#[test]
fn vtestpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 22], OperandSize::Qword)
}

