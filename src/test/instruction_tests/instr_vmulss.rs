use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 254], OperandSize::Dword)
}

#[test]
fn vmulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 962703175, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 147, 71, 175, 97, 57], OperandSize::Dword)
}

#[test]
fn vmulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 89, 253], OperandSize::Qword)
}

#[test]
fn vmulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RCX, 1413140513, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 89, 153, 33, 208, 58, 84], OperandSize::Qword)
}

#[test]
fn vmulss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 86, 254, 89, 239], OperandSize::Dword)
}

#[test]
fn vmulss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 110, 140, 89, 36, 138], OperandSize::Dword)
}

#[test]
fn vmulss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 86, 147, 89, 226], OperandSize::Qword)
}

#[test]
fn vmulss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1877606156, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 110, 139, 89, 132, 114, 12, 255, 233, 111], OperandSize::Qword)
}

