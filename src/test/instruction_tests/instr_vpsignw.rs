use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 9, 207], OperandSize::Dword)
}

#[test]
fn vpsignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 949036905, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 9, 180, 192, 105, 39, 145, 56], OperandSize::Dword)
}

#[test]
fn vpsignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 9, 204], OperandSize::Qword)
}

#[test]
fn vpsignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1682776879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 9, 4, 77, 47, 35, 77, 100], OperandSize::Qword)
}

#[test]
fn vpsignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 250], OperandSize::Dword)
}

#[test]
fn vpsignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1049655259, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 36, 253, 219, 119, 144, 62], OperandSize::Dword)
}

#[test]
fn vpsignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 9, 232], OperandSize::Qword)
}

#[test]
fn vpsignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1210751430, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 140, 73, 198, 153, 42, 72], OperandSize::Qword)
}

