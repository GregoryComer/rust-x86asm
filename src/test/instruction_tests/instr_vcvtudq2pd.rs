use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 122, 209], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1425838465, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 122, 172, 198, 129, 145, 252, 84], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 126, 138, 122, 200], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectDisplaced(RBX, 927164514, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 126, 139, 122, 131, 98, 104, 67, 55], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 122, 207], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1388952504, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 122, 180, 134, 184, 187, 201, 82], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 126, 175, 122, 240], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 156443488, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 169, 122, 28, 69, 96, 35, 83, 9], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 122, 192], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EBX, 1977934862, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 122, 139, 14, 228, 228, 117], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 126, 205, 122, 193], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 610281737, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 203, 122, 148, 95, 9, 41, 96, 36], OperandSize::Qword)
}

