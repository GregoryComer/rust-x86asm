use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 191, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1164819104, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 191, 4, 205, 160, 186, 109, 69], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 191, 193], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 191, 28, 129], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 251, 191, 241], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 191, 52, 122], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 157, 191, 191, 249], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 767750082, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 197, 137, 191, 164, 64, 194, 239, 194, 45], OperandSize::Qword)
}

