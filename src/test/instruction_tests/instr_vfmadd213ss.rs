use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 169, 211], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 169, 38], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 169, 248], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 992771590, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 169, 12, 133, 6, 126, 44, 59], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 254, 169, 251], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 73878281, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 169, 182, 9, 75, 103, 4], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 45, 148, 169, 217], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 69, 134, 169, 52, 194], OperandSize::Qword)
}

