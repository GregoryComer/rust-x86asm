use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 155, 194], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 717660372, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 12, 85, 212, 160, 198, 42], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 220], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 529708348, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 155, 170, 60, 181, 146, 31], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 222, 155, 213], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 1282550998, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 155, 146, 214, 44, 114, 76], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 165, 178, 155, 252], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 677920968, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 245, 133, 155, 188, 91, 200, 64, 104, 40], OperandSize::Qword)
}

