use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 157, 67, 243], OperandSize::Dword)
}

#[test]
fn vgetexpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 310569488, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 67, 180, 90, 16, 234, 130, 18], OperandSize::Dword)
}

#[test]
fn vgetexpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 229, 153, 67, 196], OperandSize::Qword)
}

#[test]
fn vgetexpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1168107529, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 213, 134, 67, 28, 69, 9, 232, 159, 69], OperandSize::Qword)
}

