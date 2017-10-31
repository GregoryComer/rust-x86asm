use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 528930340, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 26, 138, 36, 214, 134, 31], OperandSize::Dword)
}

#[test]
fn vbroadcastf64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RBX, 1016042482, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 26, 179, 242, 147, 143, 60], OperandSize::Qword)
}

