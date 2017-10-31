use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 94, 237], OperandSize::Dword)
}

#[test]
fn vdivsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 81521832, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 94, 154, 168, 236, 219, 4], OperandSize::Dword)
}

#[test]
fn vdivsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 94, 231], OperandSize::Qword)
}

#[test]
fn vdivsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 94, 52, 136], OperandSize::Qword)
}

#[test]
fn vdivsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 239, 185, 94, 226], OperandSize::Dword)
}

#[test]
fn vdivsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1130231167, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 199, 139, 94, 180, 206, 127, 245, 93, 67], OperandSize::Dword)
}

#[test]
fn vdivsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 151, 187, 94, 206], OperandSize::Qword)
}

#[test]
fn vdivsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 2136696237, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 167, 137, 94, 156, 218, 173, 101, 91, 127], OperandSize::Qword)
}

