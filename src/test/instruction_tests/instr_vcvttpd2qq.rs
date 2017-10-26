use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 122, 193], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 122, 38], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 122, 252], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 304988509, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 122, 172, 126, 93, 193, 45, 18], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 122, 236], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1750158626, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 122, 132, 90, 34, 77, 81, 104], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 173, 122, 196], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 122, 47], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 158, 122, 225], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 761440163, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 122, 44, 245, 163, 167, 98, 45], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 154, 122, 236], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 2012387561, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 122, 36, 181, 233, 152, 242, 119], OperandSize::Qword)
}

