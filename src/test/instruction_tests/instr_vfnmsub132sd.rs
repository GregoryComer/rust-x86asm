use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 159, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 159, 52, 210], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 159, 193], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 159, 60, 186], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 219, 159, 232], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 159, 12, 112], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 189, 186, 159, 234], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RCX, 521142858, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 229, 143, 159, 145, 74, 2, 16, 31], OperandSize::Qword)
}

