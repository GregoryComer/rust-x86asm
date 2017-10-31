use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 95, 216], OperandSize::Dword)
}

#[test]
fn vmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 225000762, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 95, 4, 69, 58, 61, 105, 13], OperandSize::Dword)
}

#[test]
fn vmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 95, 196], OperandSize::Qword)
}

#[test]
fn vmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 95, 44, 64], OperandSize::Qword)
}

#[test]
fn vmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 155, 95, 246], OperandSize::Dword)
}

#[test]
fn vmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1662552809, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 207, 141, 95, 36, 181, 233, 138, 24, 99], OperandSize::Dword)
}

#[test]
fn vmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 151, 145, 95, 245], OperandSize::Qword)
}

#[test]
fn vmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 183, 131, 95, 20, 214], OperandSize::Qword)
}

