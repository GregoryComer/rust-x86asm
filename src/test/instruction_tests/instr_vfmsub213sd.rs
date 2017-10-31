use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 171, 218], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 171, 32], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 171, 251], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 171, 46], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 252, 171, 209], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 171, 20, 217], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 149, 220, 171, 242], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RBX, 1520464045, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 165, 140, 171, 131, 173, 112, 160, 90], OperandSize::Qword)
}

