use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 159, 236], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1792374187, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 159, 152, 171, 117, 213, 106], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 159, 204], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 799478142, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 159, 36, 149, 126, 17, 167, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 255, 159, 198], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1151843133, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 159, 170, 61, 187, 167, 68], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 157, 255, 159, 219], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 922879012, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 229, 133, 159, 44, 189, 36, 4, 2, 55], OperandSize::Qword)
}

