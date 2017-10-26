use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 173, 234], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1815843909, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 173, 139, 69, 148, 59, 108], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 173, 212], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 1596972167, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 173, 130, 135, 220, 47, 95], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 253, 173, 222], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1012545563, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 173, 4, 253, 27, 56, 90, 60], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 149, 217, 173, 196], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 245, 142, 173, 44, 138], OperandSize::Qword)
}

