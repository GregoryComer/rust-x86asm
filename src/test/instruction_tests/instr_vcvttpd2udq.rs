use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 137, 120, 216], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 774602401, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 139, 120, 183, 161, 126, 43, 46], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 252, 137, 120, 212], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1220555047, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 143, 120, 20, 197, 39, 49, 192, 72], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 170, 120, 241], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 799517453, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 174, 120, 128, 13, 171, 167, 47], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 252, 170, 120, 235], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 209378817, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 173, 120, 44, 77, 1, 222, 122, 12], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 154, 120, 216], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 204, 120, 20, 154], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 252, 159, 120, 234], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 252, 203, 120, 36, 159], OperandSize::Qword)
}

