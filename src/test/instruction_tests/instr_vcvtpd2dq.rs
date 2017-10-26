use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 206], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 2046993315, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 191, 163, 163, 2, 122], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 221], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 19], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 241], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 12, 112], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 203], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 10], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 230, 195], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 290075763, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 230, 188, 95, 115, 52, 74, 17], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 255, 140, 230, 210], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 114123509, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 230, 28, 197, 245, 98, 205, 6], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 230, 230], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 552584820, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 230, 136, 116, 198, 239, 32], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 255, 170, 230, 195], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 309748347, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 255, 169, 230, 156, 95, 123, 98, 118, 18], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 188, 230, 228], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 230, 52, 199], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 255, 156, 230, 248], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RCX, 1460783977, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 230, 137, 105, 203, 17, 87], OperandSize::Qword)
}

