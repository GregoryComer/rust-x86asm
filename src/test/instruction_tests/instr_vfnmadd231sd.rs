use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 189, 225], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 204970568, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 189, 20, 141, 72, 154, 55, 12], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 189, 211], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 189, 41], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 191, 189, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 2001560251, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 189, 60, 93, 187, 98, 77, 119], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 221, 191, 189, 221], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 986574939, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 135, 189, 20, 181, 91, 240, 205, 58], OperandSize::Qword)
}

