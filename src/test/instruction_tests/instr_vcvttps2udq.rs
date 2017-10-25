use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 120, 245], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1909735050, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 120, 36, 245, 138, 62, 212, 113], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 124, 137, 120, 204], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 867112403, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 120, 156, 248, 211, 21, 175, 51], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 120, 246], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 120, 0], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 170, 120, 222], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RDX, 1843233737, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 172, 120, 162, 201, 131, 221, 109], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 158, 120, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 120, 7], OperandSize::Dword)
}

#[test]
fn vcvttps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 124, 157, 120, 250], OperandSize::Qword)
}

#[test]
fn vcvttps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1965295805, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 203, 120, 28, 157, 189, 8, 36, 117], OperandSize::Qword)
}

