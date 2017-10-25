use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 173, 251], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 173, 12, 91], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 173, 237], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1907828412, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 173, 12, 213, 188, 38, 183, 113], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 188, 173, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1992729571, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 173, 130, 227, 163, 198, 118], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 21, 241, 173, 250], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 61, 130, 173, 22], OperandSize::Qword)
}

