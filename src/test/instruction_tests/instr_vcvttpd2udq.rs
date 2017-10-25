use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 120, 216], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1320996917, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 143, 120, 164, 87, 53, 208, 188, 78], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 252, 138, 120, 242], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 252, 141, 120, 12, 123], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 170, 120, 222], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EAX, 328100117, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 169, 120, 144, 21, 105, 142, 19], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 252, 175, 120, 216], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 384078994, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 252, 171, 120, 180, 214, 146, 148, 228, 22], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 153, 120, 228], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 1840048578, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 205, 120, 134, 194, 233, 172, 109], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 252, 159, 120, 236], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 252, 207, 120, 12, 186], OperandSize::Qword)
}

