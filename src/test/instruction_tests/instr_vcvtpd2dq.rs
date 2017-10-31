use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 204], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1127604264, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 36, 69, 40, 224, 53, 67], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 196], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 445696997, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 44, 85, 229, 203, 144, 26], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 194], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 362148766, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 162, 158, 243, 149, 21], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 240], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 514819899, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 144, 59, 135, 175, 30], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 230, 218], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 230, 4, 90], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 255, 138, 230, 249], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 2060137629, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 255, 138, 230, 188, 249, 157, 52, 203, 122], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 230, 230], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 2052854011, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 230, 191, 251, 16, 92, 122], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 255, 175, 230, 219], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 172, 230, 4, 73], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 157, 230, 233], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EBX, 803848076, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 230, 171, 140, 191, 233, 47], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 255, 251, 230, 240], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RSI, 329395133, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 206, 230, 158, 189, 43, 162, 19], OperandSize::Qword)
}

