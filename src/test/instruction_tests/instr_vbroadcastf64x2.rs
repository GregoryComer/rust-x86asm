use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 26, 23], OperandSize::Dword)
}

#[test]
fn vbroadcastf64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM21)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 174, 26, 41], OperandSize::Qword)
}

