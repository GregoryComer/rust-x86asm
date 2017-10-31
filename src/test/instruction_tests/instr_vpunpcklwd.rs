use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 97, 223], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 697502309, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 97, 180, 155, 101, 10, 147, 41], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 97, 245], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 1261247000, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 97, 162, 24, 26, 45, 75], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 97, 238], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 2049458441, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 97, 148, 70, 9, 65, 40, 122], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 97, 228], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 440146224, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 97, 60, 253, 48, 25, 60, 26], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 97, 207], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 312877040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 97, 169, 240, 31, 166, 18], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 77, 140, 97, 251], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1104935483, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 21, 142, 97, 60, 157, 59, 250, 219, 65], OperandSize::Qword)
}

