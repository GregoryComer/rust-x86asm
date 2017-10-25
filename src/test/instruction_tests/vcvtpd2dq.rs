use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 211], OperandSize::Dword)
}

fn vcvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 1260065029, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 178, 5, 17, 27, 75], OperandSize::Dword)
}

fn vcvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 231], OperandSize::Qword)
}

fn vcvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 44, 241], OperandSize::Qword)
}

fn vcvtpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 204], OperandSize::Dword)
}

fn vcvtpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 4, 250], OperandSize::Dword)
}

fn vcvtpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 249], OperandSize::Qword)
}

fn vcvtpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1857867051, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 52, 85, 43, 205, 188, 110], OperandSize::Qword)
}

fn vcvtpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 230, 244], OperandSize::Dword)
}

fn vcvtpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 202986843, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 230, 142, 91, 85, 25, 12], OperandSize::Dword)
}

fn vcvtpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 255, 143, 230, 213], OperandSize::Qword)
}

fn vcvtpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1812710959, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 230, 12, 157, 47, 198, 11, 108], OperandSize::Qword)
}

fn vcvtpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 230, 235], OperandSize::Dword)
}

fn vcvtpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1374449103, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 230, 148, 158, 207, 109, 236, 81], OperandSize::Dword)
}

fn vcvtpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 255, 173, 230, 255], OperandSize::Qword)
}

fn vcvtpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectDisplaced(RCX, 1459883217, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 174, 230, 185, 209, 12, 4, 87], OperandSize::Qword)
}

fn vcvtpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 219, 230, 199], OperandSize::Dword)
}

fn vcvtpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 230, 28, 123], OperandSize::Dword)
}

fn vcvtpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 255, 185, 230, 242], OperandSize::Qword)
}

fn vcvtpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM8)), operand2: Some(IndirectDisplaced(RDX, 679326977, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 255, 205, 230, 130, 1, 181, 125, 40], OperandSize::Qword)
}

