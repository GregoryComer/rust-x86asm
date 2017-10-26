use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 91, 42], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 91, 35], OperandSize::Qword)
}

