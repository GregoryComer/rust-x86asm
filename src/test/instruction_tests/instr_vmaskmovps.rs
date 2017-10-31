use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 44, 12, 91], OperandSize::Dword)
}

#[test]
fn vmaskmovps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 44, 36, 142], OperandSize::Qword)
}

#[test]
fn vmaskmovps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 1140633179, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 44, 128, 91, 174, 252, 67], OperandSize::Dword)
}

#[test]
fn vmaskmovps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 44, 4, 147], OperandSize::Qword)
}

#[test]
fn vmaskmovps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 46, 60, 134], OperandSize::Dword)
}

#[test]
fn vmaskmovps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 876814899, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 46, 180, 191, 51, 34, 67, 52], OperandSize::Qword)
}

#[test]
fn vmaskmovps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectDisplaced(ECX, 259507747, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 46, 145, 35, 198, 119, 15], OperandSize::Dword)
}

#[test]
fn vmaskmovps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1532890708, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 46, 28, 149, 84, 14, 94, 91], OperandSize::Qword)
}

