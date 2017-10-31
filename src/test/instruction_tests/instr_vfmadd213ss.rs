use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 169, 255], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 169, 46], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 169, 238], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 2108410498, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 169, 20, 133, 130, 202, 171, 125], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 218, 169, 228], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2003884799, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 169, 36, 205, 255, 218, 112, 119], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 21, 191, 169, 203], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 545158657, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 61, 142, 169, 44, 93, 1, 118, 126, 32], OperandSize::Qword)
}

