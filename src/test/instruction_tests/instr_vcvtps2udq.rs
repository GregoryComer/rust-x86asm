use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 121, 221], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 426382823, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 121, 186, 231, 21, 106, 25], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 124, 141, 121, 217], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 124, 143, 121, 60, 72], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 121, 207], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 121, 52, 246], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 173, 121, 230], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectDisplaced(RAX, 2040355501, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 171, 121, 144, 173, 90, 157, 121], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 217, 121, 247], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1848227598, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 121, 44, 221, 14, 183, 41, 110], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 251, 121, 228], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1397362587, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 124, 204, 121, 140, 223, 155, 15, 74, 83], OperandSize::Qword)
}

