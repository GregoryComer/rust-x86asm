use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDX, 1565404251, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 26, 170, 91, 44, 78, 93], OperandSize::Dword)
}

#[test]
fn vbroadcastf64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1681267214, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 26, 12, 189, 14, 26, 54, 100], OperandSize::Qword)
}

