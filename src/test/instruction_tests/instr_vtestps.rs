use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 236], OperandSize::Dword)
}

#[test]
fn vtestps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 12, 200], OperandSize::Dword)
}

#[test]
fn vtestps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 199], OperandSize::Qword)
}

#[test]
fn vtestps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 892933764, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 158, 132, 22, 57, 53], OperandSize::Qword)
}

#[test]
fn vtestps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 203], OperandSize::Dword)
}

#[test]
fn vtestps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 2089449569, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 4, 133, 97, 120, 138, 124], OperandSize::Dword)
}

#[test]
fn vtestps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 216], OperandSize::Qword)
}

#[test]
fn vtestps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RAX, 1326478813, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 176, 221, 117, 16, 79], OperandSize::Qword)
}

