use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 105, 228], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 105, 4, 123], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 105, 212], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 387234211, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 105, 179, 163, 185, 20, 23], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 105, 232], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 54353513, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 105, 143, 105, 94, 61, 3], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 105, 248], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 105, 0], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 105, 235], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 105, 32], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 138, 105, 210], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1720167277, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 117, 141, 105, 180, 187, 109, 171, 135, 102], OperandSize::Qword)
}

