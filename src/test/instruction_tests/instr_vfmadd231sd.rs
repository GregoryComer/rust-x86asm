use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 185, 207], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1920440944, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 185, 164, 79, 112, 154, 119, 114], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 185, 245], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 185, 52, 248], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 252, 185, 251], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 185, 44, 209], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 181, 189, 185, 198], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 428031840, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 131, 185, 172, 176, 96, 63, 131, 25], OperandSize::Qword)
}

