use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 16, 222], OperandSize::Qword)
}

#[test]
fn vmovss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 832382091, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 176, 139, 36, 157, 49], OperandSize::Dword)
}

#[test]
fn vmovss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 22], OperandSize::Qword)
}

#[test]
fn vmovss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 118, 137, 16, 232], OperandSize::Dword)
}

#[test]
fn vmovss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 46, 131, 16, 193], OperandSize::Qword)
}

#[test]
fn vmovss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1183582040, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 16, 156, 71, 88, 7, 140, 70], OperandSize::Dword)
}

#[test]
fn vmovss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 16, 10], OperandSize::Qword)
}

#[test]
fn vmovss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 16, 235], OperandSize::Dword)
}

#[test]
fn vmovss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 16, 204], OperandSize::Qword)
}

#[test]
fn vmovss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 44, 176], OperandSize::Dword)
}

#[test]
fn vmovss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(RDX, 412400513, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 178, 129, 187, 148, 24], OperandSize::Qword)
}

#[test]
fn vmovss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 78, 138, 16, 210], OperandSize::Dword)
}

#[test]
fn vmovss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 14, 139, 16, 232], OperandSize::Qword)
}

#[test]
fn vmovss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 12, 214], OperandSize::Dword)
}

#[test]
fn vmovss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 8, 17, 51], OperandSize::Qword)
}

