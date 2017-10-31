use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 26, 19], OperandSize::Dword)
}

#[test]
fn vbroadcastf64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 169, 26, 19], OperandSize::Qword)
}

