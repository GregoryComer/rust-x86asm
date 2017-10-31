use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 120, 223], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1863874331, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 120, 60, 197, 27, 119, 24, 111], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 124, 141, 120, 226], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1566770506, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 120, 36, 85, 74, 5, 99, 93], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 120, 250], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 479288663, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 120, 140, 250, 87, 93, 145, 28], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 124, 175, 120, 213], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1400418117, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 120, 52, 205, 69, 175, 120, 83], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 159, 120, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ESI, 1888789686, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 120, 158, 182, 164, 148, 112], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 124, 158, 120, 243], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectDisplaced(RDI, 543843447, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 203, 120, 175, 119, 100, 106, 32], OperandSize::Qword)
}

