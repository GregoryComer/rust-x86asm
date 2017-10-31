use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 214], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1527386707, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 153, 44, 69, 83, 18, 10, 91], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 153, 227], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 153, 12, 82], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 221, 153, 195], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1537895348, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 153, 156, 186, 180, 107, 170, 91], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 189, 221, 153, 195], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1216560805, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 139, 153, 12, 117, 165, 62, 131, 72], OperandSize::Qword)
}

