use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 189, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 189, 20, 246], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 189, 236], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1687975283, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 189, 139, 115, 117, 156, 100], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 222, 189, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 394932710, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 189, 44, 189, 230, 49, 138, 23], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 165, 246, 189, 253], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1312300430, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 221, 131, 189, 188, 83, 142, 29, 56, 78], OperandSize::Qword)
}

