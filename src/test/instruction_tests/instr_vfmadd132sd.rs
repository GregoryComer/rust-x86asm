use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 153, 225], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 231524380, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 156, 178, 28, 200, 204, 13], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 153, 251], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 44, 211], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 190, 153, 228], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 239979526, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 153, 185, 6, 204, 77, 14], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 197, 245, 153, 218], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 197, 129, 153, 20, 75], OperandSize::Qword)
}

