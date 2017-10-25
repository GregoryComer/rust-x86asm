use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 121, 233], OperandSize::Dword)
}

fn vcvtpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 352198871, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 121, 20, 77, 215, 32, 254, 20], OperandSize::Dword)
}

fn vcvtpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 253, 142, 121, 211], OperandSize::Qword)
}

fn vcvtpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 800876025, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 121, 140, 138, 249, 101, 188, 47], OperandSize::Qword)
}

fn vcvtpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 121, 194], OperandSize::Dword)
}

fn vcvtpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1876276963, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 121, 28, 149, 227, 182, 213, 111], OperandSize::Dword)
}

fn vcvtpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 253, 173, 121, 232], OperandSize::Qword)
}

fn vcvtpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 11006824, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 171, 121, 148, 153, 104, 243, 167, 0], OperandSize::Qword)
}

fn vcvtpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 188, 121, 193], OperandSize::Dword)
}

fn vcvtpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 268535322, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 121, 20, 125, 26, 134, 1, 16], OperandSize::Dword)
}

fn vcvtpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 186, 121, 196], OperandSize::Qword)
}

fn vcvtpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UQQ, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectDisplaced(RDX, 209495448, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 121, 154, 152, 165, 124, 12], OperandSize::Qword)
}

