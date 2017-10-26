use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 253, 45, 214], OperandSize::Dword)
}

#[test]
fn vscalefss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 45, 20, 94], OperandSize::Dword)
}

#[test]
fn vscalefss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 29, 223, 45, 202], OperandSize::Qword)
}

#[test]
fn vscalefss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 117, 140, 45, 28, 143], OperandSize::Qword)
}

