use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 187, 203], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 187, 22], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 187, 220], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 187, 36, 136], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 251, 187, 205], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1804677815, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 187, 188, 80, 183, 50, 145, 107], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 157, 249, 187, 200], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1229063518, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 149, 129, 187, 148, 120, 94, 5, 66, 73], OperandSize::Qword)
}

