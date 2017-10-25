use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 191, 245], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 191, 26], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 191, 233], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 107073863, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 191, 52, 149, 71, 209, 97, 6], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 222, 191, 209], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 191, 28, 187], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 133, 147, 191, 198], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1836212313, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 189, 132, 191, 60, 197, 89, 96, 114, 109], OperandSize::Qword)
}

