use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 154, 67, 255], OperandSize::Dword)
}

#[test]
fn vgetexpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 1102472610, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 67, 136, 162, 101, 182, 65], OperandSize::Dword)
}

#[test]
fn vgetexpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 85, 148, 67, 213], OperandSize::Qword)
}

#[test]
fn vgetexpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 5, 142, 67, 4, 90], OperandSize::Qword)
}

