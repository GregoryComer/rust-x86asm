use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 187, 233], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 537881919, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 187, 168, 63, 109, 15, 32], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 187, 210], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 187, 12, 215], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 188, 187, 250], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 187, 52, 193], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 189, 147, 187, 255], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 221, 130, 187, 56], OperandSize::Qword)
}

