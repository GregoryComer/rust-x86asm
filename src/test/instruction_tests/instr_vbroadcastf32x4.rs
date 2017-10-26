use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 325882819, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 26, 159, 195, 147, 108, 19], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM24)), operand2: Some(IndirectDisplaced(RCX, 1437175990, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 26, 129, 182, 144, 169, 85], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 662111956, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 26, 172, 78, 212, 6, 119, 39], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 413202852, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 26, 132, 129, 164, 249, 160, 24], OperandSize::Qword)
}

