use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 96, 205], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 900868425, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 28, 85, 73, 41, 178, 53], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 96, 200], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 28, 202], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 96, 210], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 96, 32], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 96, 197], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 96, 20, 134], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 96, 202], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 96, 17], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 61, 137, 96, 254], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 29, 142, 96, 28, 218], OperandSize::Qword)
}

