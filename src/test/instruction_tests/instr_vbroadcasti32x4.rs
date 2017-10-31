use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDX, 507551636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 90, 154, 148, 159, 64, 30], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 2049775100, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 90, 172, 216, 252, 21, 45, 122], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 542891258, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 90, 44, 213, 250, 220, 91, 32], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1802237714, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 90, 52, 69, 18, 247, 107, 107], OperandSize::Qword)
}

