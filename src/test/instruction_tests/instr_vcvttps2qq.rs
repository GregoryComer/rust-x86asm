use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 122, 248], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 990882087, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 122, 132, 218, 39, 169, 15, 59], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 125, 142, 122, 235], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 357285616, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 122, 128, 240, 190, 75, 21], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 122, 206], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EBX, 261111355, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 122, 179, 59, 62, 144, 15], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 125, 172, 122, 234], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM9)), operand2: Some(IndirectDisplaced(RDX, 102745091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 172, 122, 138, 3, 196, 31, 6], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 155, 122, 245], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 122, 12, 195], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 125, 156, 122, 234], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 125, 201, 122, 52, 185], OperandSize::Qword)
}

