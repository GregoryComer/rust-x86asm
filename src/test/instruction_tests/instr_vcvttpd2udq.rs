use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 139, 120, 197], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1814665693, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 120, 28, 197, 221, 153, 41, 108], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 252, 139, 120, 232], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1534704107, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 252, 141, 120, 164, 219, 235, 185, 121, 91], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 170, 120, 238], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 174, 120, 4, 192], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 252, 174, 120, 209], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 252, 171, 120, 52, 113], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 159, 120, 213], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 765627818, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 205, 120, 153, 170, 141, 162, 45], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 252, 155, 120, 244], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 997651501, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 202, 120, 140, 64, 45, 244, 118, 59], OperandSize::Qword)
}

