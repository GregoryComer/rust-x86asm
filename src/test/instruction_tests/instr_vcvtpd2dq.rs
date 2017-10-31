use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 193], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 60, 248], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 222], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 2041558528, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 148, 223, 0, 182, 175, 121], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 226], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 1668747824, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 160, 48, 18, 119, 99], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 200], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 623403484, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 140, 64, 220, 97, 40, 37], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 230, 250], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 230, 11], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 255, 143, 230, 199], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 142, 230, 4, 83], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 230, 214], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1919166145, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 230, 156, 198, 193, 38, 100, 114], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 255, 173, 230, 231], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RDI, 2035259138, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 255, 174, 230, 175, 2, 151, 79, 121], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 218, 230, 202], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 203, 230, 60, 151], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 255, 189, 230, 241], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 193300582, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 204, 230, 36, 189, 102, 136, 133, 11], OperandSize::Qword)
}

