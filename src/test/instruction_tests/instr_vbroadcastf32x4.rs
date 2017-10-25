use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 983831746, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 26, 28, 245, 194, 20, 164, 58], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1202619227, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 26, 180, 179, 91, 131, 174, 71], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 26, 12, 87], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 26, 44, 214], OperandSize::Qword)
}

