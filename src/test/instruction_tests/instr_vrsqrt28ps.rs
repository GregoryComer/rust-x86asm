use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 204, 196], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(ESI, 1660623545, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 204, 166, 185, 26, 251, 98], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 204, 60, 182], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 125, 159, 204, 222], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM22)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 204, 55], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 204, 28, 65], OperandSize::Qword)
}

