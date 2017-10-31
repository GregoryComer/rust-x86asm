use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 157, 67, 239], OperandSize::Dword)
}

#[test]
fn vgetexpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1664045789, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 67, 156, 147, 221, 82, 47, 99], OperandSize::Dword)
}

#[test]
fn vgetexpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 237, 158, 67, 208], OperandSize::Qword)
}

#[test]
fn vgetexpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 165, 134, 67, 4, 217], OperandSize::Qword)
}

