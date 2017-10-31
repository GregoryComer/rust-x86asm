use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 121, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 121, 59], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 124, 139, 121, 230], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 858724772, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 121, 12, 253, 164, 25, 47, 51], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 121, 219], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EDX, 1041181191, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 121, 162, 7, 42, 15, 62], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 124, 170, 121, 216], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM20)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 173, 121, 39], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 188, 121, 246], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDI, 1364618292, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 121, 175, 52, 108, 86, 81], OperandSize::Dword)
}

#[test]
fn vcvtps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 124, 156, 121, 212], OperandSize::Qword)
}

#[test]
fn vcvtps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM9)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 204, 121, 11], OperandSize::Qword)
}

