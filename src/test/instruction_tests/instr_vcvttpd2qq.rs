use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 122, 217], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 122, 36, 129], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 253, 142, 122, 233], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 139, 122, 60, 83], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 122, 230], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(ECX, 1934681835, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 122, 161, 235, 230, 80, 115], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 253, 172, 122, 210], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RCX, 1164707301, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 122, 145, 229, 5, 108, 69], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 157, 122, 198], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ESI, 1604866551, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 122, 174, 247, 81, 168, 95], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 153, 122, 251], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 186440297, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 203, 122, 28, 189, 105, 218, 28, 11], OperandSize::Qword)
}

