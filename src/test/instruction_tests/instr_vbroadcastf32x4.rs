use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDX, 1845975949, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 26, 186, 141, 91, 7, 110], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM27)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 175, 26, 30], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 26, 16], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 26, 4, 95], OperandSize::Qword)
}

