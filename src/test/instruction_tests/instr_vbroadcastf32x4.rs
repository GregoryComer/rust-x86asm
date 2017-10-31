use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1178895961, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 26, 28, 85, 89, 134, 68, 70], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM10)), operand2: Some(IndirectDisplaced(RCX, 34863451, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 171, 26, 145, 91, 249, 19, 2], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 337061470, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 26, 60, 253, 94, 38, 23, 20], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 2136203625, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 202, 26, 180, 90, 105, 225, 83, 127], OperandSize::Qword)
}

