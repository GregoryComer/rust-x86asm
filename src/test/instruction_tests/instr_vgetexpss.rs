use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 154, 67, 227], OperandSize::Dword)
}

#[test]
fn vgetexpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 67, 20, 75], OperandSize::Dword)
}

#[test]
fn vgetexpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 37, 148, 67, 231], OperandSize::Qword)
}

#[test]
fn vgetexpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 874549643, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 29, 142, 67, 164, 138, 139, 145, 32, 52], OperandSize::Qword)
}

