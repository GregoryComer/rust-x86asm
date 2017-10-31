use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 11], OperandSize::Dword)
}

#[test]
fn vbroadcasti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RBX, 529869520, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 147, 208, 42, 149, 31], OperandSize::Qword)
}

