use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 95, 246], OperandSize::Dword)
}

#[test]
fn vmaxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 95, 58], OperandSize::Dword)
}

#[test]
fn vmaxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 95, 196], OperandSize::Qword)
}

#[test]
fn vmaxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 95, 20, 144], OperandSize::Qword)
}

#[test]
fn vmaxss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 86, 158, 95, 200], OperandSize::Dword)
}

#[test]
fn vmaxss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1996752254, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 118, 140, 95, 28, 189, 126, 5, 4, 119], OperandSize::Dword)
}

#[test]
fn vmaxss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 6, 149, 95, 198], OperandSize::Qword)
}

#[test]
fn vmaxss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 102, 133, 95, 60, 73], OperandSize::Qword)
}

