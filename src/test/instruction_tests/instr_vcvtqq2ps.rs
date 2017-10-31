use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 91, 230], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 91, 1], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 252, 137, 91, 228], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 534333431, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 252, 140, 91, 44, 85, 247, 71, 217, 31], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 171, 91, 244], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 173, 91, 27], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 252, 170, 91, 253], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 149821019, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 91, 167, 91, 22, 238, 8], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 218, 91, 251], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 128733073, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 204, 91, 12, 93, 145, 79, 172, 7], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 252, 252, 91, 211], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM28)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 252, 201, 91, 32], OperandSize::Qword)
}

