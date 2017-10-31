use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 191, 225], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 145592349, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 191, 28, 77, 29, 144, 173, 8], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 191, 236], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1092813759, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 191, 148, 184, 191, 3, 35, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 159, 191, 229], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 191, 60, 187], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 197, 250, 191, 218], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 191, 31], OperandSize::Qword)
}

