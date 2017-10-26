use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 120, 235], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1051396575, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 120, 4, 85, 223, 9, 171, 62], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 141, 120, 244], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 739997973, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 142, 120, 28, 205, 21, 121, 27, 44], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 120, 232], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1984639460, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 120, 140, 81, 228, 49, 75, 118], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 173, 120, 249], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM10)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 170, 120, 23], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 157, 120, 207], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 2134973002, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 120, 148, 255, 74, 26, 65, 127], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 124, 155, 120, 199], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 205, 120, 60, 87], OperandSize::Qword)
}

