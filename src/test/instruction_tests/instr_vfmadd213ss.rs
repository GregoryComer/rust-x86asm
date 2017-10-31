use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 169, 236], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 169, 1], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 169, 236], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 169, 22], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 191, 169, 193], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 169, 42], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 85, 222, 169, 244], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RCX, 2057347647, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 101, 131, 169, 153, 63, 162, 160, 122], OperandSize::Qword)
}

