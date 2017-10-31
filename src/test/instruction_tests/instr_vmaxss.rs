use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 95, 213], OperandSize::Dword)
}

#[test]
fn vmaxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 95, 44, 200], OperandSize::Dword)
}

#[test]
fn vmaxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 95, 200], OperandSize::Qword)
}

#[test]
fn vmaxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 95, 56], OperandSize::Qword)
}

#[test]
fn vmaxss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 78, 156, 95, 234], OperandSize::Dword)
}

#[test]
fn vmaxss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 96068793, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 118, 139, 95, 60, 189, 185, 228, 185, 5], OperandSize::Dword)
}

#[test]
fn vmaxss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 54, 145, 95, 218], OperandSize::Qword)
}

#[test]
fn vmaxss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 38, 133, 95, 1], OperandSize::Qword)
}

