use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 96, 229], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 606101938, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 96, 28, 133, 178, 97, 32, 36], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 204], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 96, 25], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 96, 230], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 453233641, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 96, 184, 233, 203, 3, 27], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 96, 212], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1153373185, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 96, 60, 157, 1, 20, 191, 68], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 96, 231], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 96, 38], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 29, 143, 96, 193], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 104709672, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 93, 137, 96, 44, 213, 40, 190, 61, 6], OperandSize::Qword)
}

