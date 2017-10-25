use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 187, 244], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 187, 52, 147], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 187, 244], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 187, 40], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 220, 187, 242], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1085292386, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 187, 36, 133, 98, 63, 176, 64], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 37, 178, 187, 254], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RSI, 865496884, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 13, 142, 187, 158, 52, 111, 150, 51], OperandSize::Qword)
}

