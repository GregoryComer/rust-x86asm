use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 194], OperandSize::Dword)
}

#[test]
fn vmovsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 16, 229], OperandSize::Qword)
}

#[test]
fn vmovsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 681565538, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 148, 155, 98, 221, 159, 40], OperandSize::Dword)
}

#[test]
fn vmovsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 276371718, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 4, 93, 6, 25, 121, 16], OperandSize::Qword)
}

#[test]
fn vmovsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 239, 143, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 135, 131, 16, 242], OperandSize::Qword)
}

#[test]
fn vmovsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 411842857, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 140, 16, 156, 70, 41, 57, 140, 24], OperandSize::Dword)
}

#[test]
fn vmovsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 16, 22], OperandSize::Qword)
}

#[test]
fn vmovsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 16, 217], OperandSize::Dword)
}

#[test]
fn vmovsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 16, 249], OperandSize::Qword)
}

#[test]
fn vmovsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 60, 70], OperandSize::Dword)
}

#[test]
fn vmovsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectDisplaced(RDX, 1911100961, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 146, 33, 22, 233, 113], OperandSize::Qword)
}

#[test]
fn vmovsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 247, 142, 16, 230], OperandSize::Dword)
}

#[test]
fn vmovsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 159, 133, 16, 252], OperandSize::Qword)
}

#[test]
fn vmovsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 439856134, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 188, 159, 6, 172, 55, 26], OperandSize::Dword)
}

#[test]
fn vmovsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 8, 17, 18], OperandSize::Qword)
}

