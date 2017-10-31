use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 120, 249], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 245896850, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 120, 148, 254, 146, 22, 168, 14], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 124, 139, 120, 244], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 998996558, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 137, 120, 60, 125, 78, 122, 139, 59], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 120, 225], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1764541601, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 120, 44, 189, 161, 196, 44, 105], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 173, 120, 200], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 120, 4, 139], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 153, 120, 238], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 414553019, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 120, 188, 240, 187, 147, 181, 24], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 157, 120, 248], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 491750518, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 206, 120, 132, 241, 118, 132, 79, 29], OperandSize::Qword)
}

