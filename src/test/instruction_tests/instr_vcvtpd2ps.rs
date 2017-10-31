use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 235], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 879163735, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 188, 198, 87, 249, 102, 52], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 192], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 22670470, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 132, 79, 134, 236, 89, 1], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 210], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 56], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 198], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 818590553, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 188, 179, 89, 179, 202, 48], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 90, 244], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 2070737580, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 90, 160, 172, 242, 108, 123], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 253, 142, 90, 243], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1401851813, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 139, 90, 132, 202, 165, 143, 142, 83], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 90, 232], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 90, 28, 90], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 253, 172, 90, 208], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 172, 90, 36, 86], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 157, 90, 235], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 90, 3], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 191, 90, 192], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 988873572, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 90, 28, 221, 100, 3, 241, 58], OperandSize::Qword)
}

