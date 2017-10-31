use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 90, 40], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 170, 90, 44, 70], OperandSize::Qword)
}

#[test]
fn vbroadcasti64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(ESI, 956454634, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 90, 182, 234, 86, 2, 57], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RAX, 1870461917, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 90, 136, 221, 251, 124, 111], OperandSize::Qword)
}

