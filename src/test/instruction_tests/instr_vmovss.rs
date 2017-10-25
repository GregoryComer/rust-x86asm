use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 16, 239], OperandSize::Dword)
}

#[test]
fn vmovss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 16, 206], OperandSize::Qword)
}

#[test]
fn vmovss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 46], OperandSize::Dword)
}

#[test]
fn vmovss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1214415550, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 188, 187, 190, 130, 98, 72], OperandSize::Qword)
}

#[test]
fn vmovss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 118, 142, 16, 194], OperandSize::Dword)
}

#[test]
fn vmovss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 126, 134, 16, 227], OperandSize::Qword)
}

#[test]
fn vmovss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 16, 57], OperandSize::Dword)
}

#[test]
fn vmovss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 2051697545, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 16, 180, 176, 137, 107, 74, 122], OperandSize::Qword)
}

#[test]
fn vmovss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 16, 203], OperandSize::Dword)
}

#[test]
fn vmovss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 16, 211], OperandSize::Qword)
}

#[test]
fn vmovss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1017435899, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 4, 125, 251, 214, 164, 60], OperandSize::Dword)
}

#[test]
fn vmovss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(RCX, 589640156, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 169, 220, 49, 37, 35], OperandSize::Qword)
}

#[test]
fn vmovss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 110, 140, 16, 234], OperandSize::Dword)
}

#[test]
fn vmovss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 6, 129, 16, 251], OperandSize::Qword)
}

#[test]
fn vmovss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 58], OperandSize::Dword)
}

#[test]
fn vmovss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(RCX, 1964054403, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 8, 17, 145, 131, 23, 17, 117], OperandSize::Qword)
}

