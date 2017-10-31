use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 155, 204, 213], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 169193110, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 204, 132, 243, 150, 174, 21, 10], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 204, 4, 126], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 253, 158, 204, 235], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RCX, 1151960120, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 201, 204, 177, 56, 132, 169, 68], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 786197798, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 221, 204, 188, 177, 38, 109, 220, 46], OperandSize::Qword)
}

