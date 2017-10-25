use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 120, 229], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 120, 59], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 120, 210], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM22)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 120, 54], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 120, 224], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1639397763, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 120, 188, 88, 131, 57, 183, 97], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 253, 172, 120, 195], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectDisplaced(RCX, 2042246683, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 174, 120, 145, 27, 54, 186, 121], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 158, 120, 207], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1230727039, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 120, 148, 70, 127, 103, 91, 73], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 253, 155, 120, 229], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 202, 120, 20, 187], OperandSize::Qword)
}

