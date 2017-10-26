use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 169, 252], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 940088661, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 169, 167, 85, 157, 8, 56], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 169, 247], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 915945442, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 169, 4, 77, 226, 55, 152, 54], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 153, 169, 239], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 169, 12, 211], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 237, 145, 169, 226], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 104283730, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 131, 169, 172, 182, 82, 62, 55, 6], OperandSize::Qword)
}

