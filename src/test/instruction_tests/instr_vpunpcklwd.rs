use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 97, 215], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 693793788, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 97, 156, 251, 252, 115, 90, 41], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 97, 199], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 97, 49], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 97, 211], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 97, 34], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 97, 193], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 573758333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 97, 188, 178, 125, 219, 50, 34], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 97, 251], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 97, 14], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 85, 132, 97, 194], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 1968398068, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 97, 152, 244, 94, 83, 117], OperandSize::Qword)
}

