use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 171, 227], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1552292648, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 171, 188, 146, 40, 27, 134, 92], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 171, 251], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 171, 12, 145], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 159, 171, 229], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 370800928, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 171, 184, 32, 249, 25, 22], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 215, 171, 255], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 45, 143, 171, 44, 203], OperandSize::Qword)
}

