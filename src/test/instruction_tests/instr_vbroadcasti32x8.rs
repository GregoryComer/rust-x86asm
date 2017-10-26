use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1542763501, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 91, 132, 185, 237, 179, 244, 91], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1653270002, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 91, 44, 157, 242, 229, 138, 98], OperandSize::Qword)
}

