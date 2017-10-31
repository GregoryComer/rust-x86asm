use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1839475282, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 91, 60, 181, 82, 42, 164, 109], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1787174218, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 91, 28, 117, 74, 29, 134, 106], OperandSize::Qword)
}

