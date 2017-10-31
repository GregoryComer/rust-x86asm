use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 122, 212], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 122, 60, 70], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 125, 141, 122, 252], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 5215816, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 141, 122, 180, 186, 72, 150, 79, 0], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 122, 254], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EBX, 719649072, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 122, 187, 48, 249, 228, 42], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 125, 171, 122, 230], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 337201529, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 122, 180, 137, 121, 73, 25, 20], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 154, 122, 242], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 470855021, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 122, 172, 223, 109, 173, 16, 28], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 125, 156, 122, 224], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 500083119, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 125, 201, 122, 172, 71, 175, 169, 206, 29], OperandSize::Qword)
}

