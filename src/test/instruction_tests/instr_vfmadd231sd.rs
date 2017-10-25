use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 185, 217], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1843874523, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 185, 132, 134, 219, 74, 231, 109], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 185, 204], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 925510980, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 185, 170, 68, 45, 42, 55], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 188, 185, 216], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 185, 30], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 221, 146, 185, 194], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 50098076, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 237, 138, 185, 28, 93, 156, 111, 252, 2], OperandSize::Qword)
}

