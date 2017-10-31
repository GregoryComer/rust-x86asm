use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 217], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 22], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 212], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 7], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 201], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 41], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 213], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RCX, 516414043, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 153, 91, 218, 199, 30], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 90, 252], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 90, 62], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 253, 138, 90, 197], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 141, 90, 60, 74], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 90, 227], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 953960739, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 90, 180, 178, 35, 73, 220, 56], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 253, 169, 90, 221], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 253, 175, 90, 20, 134], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 252, 90, 197], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 90, 26], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 254, 90, 202], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 204703108, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 205, 90, 12, 117, 132, 133, 51, 12], OperandSize::Qword)
}

