use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 246], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1140645048, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 60, 149, 184, 220, 252, 67], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 233], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 432197853, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 230, 12, 205, 221, 208, 194, 25], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 231], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1366677288, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 148, 66, 40, 215, 117, 81], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 197], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1493109441, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 230, 60, 197, 193, 10, 255, 88], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 230, 195], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 230, 58], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 255, 137, 230, 220], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM23)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 255, 137, 230, 59], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 169, 230, 201], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 230, 4, 193], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 255, 170, 230, 235], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 255, 169, 230, 46], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 217, 230, 229], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 230, 36, 70], OperandSize::Dword)
}

#[test]
fn vcvtpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 255, 249, 230, 249], OperandSize::Qword)
}

#[test]
fn vcvtpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RAX, 953303751, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 230, 184, 199, 66, 210, 56], OperandSize::Qword)
}

