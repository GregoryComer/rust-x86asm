use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(ESI, 2093319335, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 166, 167, 132, 197, 124], OperandSize::Dword)
}

#[test]
fn vbroadcastf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 32], OperandSize::Qword)
}

