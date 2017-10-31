use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 122, 246], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 753648372, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 122, 20, 181, 244, 194, 235, 44], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 253, 140, 122, 206], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM21)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 142, 122, 46], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 122, 203], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ESI, 1605869302, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 122, 150, 246, 158, 183, 95], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 253, 169, 122, 225], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM29)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 175, 122, 41], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 159, 122, 196], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1725879522, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 122, 172, 240, 226, 212, 222, 102], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 253, 154, 122, 235], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RCX, 1921315626, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 201, 122, 153, 42, 243, 132, 114], OperandSize::Qword)
}

