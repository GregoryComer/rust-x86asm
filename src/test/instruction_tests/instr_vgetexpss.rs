use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 153, 67, 251], OperandSize::Dword)
}

#[test]
fn vgetexpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 67, 28, 78], OperandSize::Dword)
}

#[test]
fn vgetexpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 85, 155, 67, 206], OperandSize::Qword)
}

#[test]
fn vgetexpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1553998712, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 93, 143, 67, 188, 190, 120, 35, 160, 92], OperandSize::Qword)
}

