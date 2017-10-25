use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 121, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 121, 59], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 124, 138, 121, 228], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM19)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 139, 121, 31], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 121, 253], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 447216082, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 121, 4, 245, 210, 249, 167, 26], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 124, 174, 121, 241], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 173, 121, 52, 154], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 223, 121, 240], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1541482433, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 121, 164, 176, 193, 39, 225, 91], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 124, 185, 121, 198], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 205, 121, 33], OperandSize::Qword)
}

